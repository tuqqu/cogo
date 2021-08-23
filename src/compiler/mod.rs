use std::mem;

use self::flow::ControlFlow;
pub(crate) use self::opcode::OpCode;
use self::unit::{CompilationUnit as CUnit, FuncUnit, PackageUnit, Param};
use self::value::{ValType, Value};
use crate::compiler::scope::Scope;
use crate::lex::lexeme::{Lexeme, Pos, Token};
use crate::lex::Lexer;
use crate::vm::CUnitFrame;

mod flow;
mod opcode;
mod scope;
pub(crate) mod unit;
pub(crate) mod value;

pub fn compile(src: String) -> CUnitFrame {
    let mut lexer = Lexer::new(src);
    let (lexemes, errors) = lexer.lex(); //FIXME: handle errs

    if !errors.is_empty() {
        for error in errors {
            eprintln!("\x1b[0;31m{}\x1b[0m", error);
        }
        std::process::exit(1);
    }

    let mut parser = Compiler::new(lexemes);
    let mut cunit = parser.compile();
    cunit.chunk_mut().write(OpCode::Exit, Pos(0, 0));

    eprintln!("\x1b[0;34m{:#?}\x1b[0m", cunit.chunk());

    CUnitFrame::new(cunit)
}

struct Compiler<'a> {
    lexemes: &'a [Lexeme],
    current: usize,
    cunit: CUnit,
    errs: Vec<String>,
    panic: bool,
    scope: Scope,
    control_flow: ControlFlow,
    cur_package: Option<Package>,
}

type ParseCallback<T> = fn(&mut T, bool);
type ParseRule<T> = (
    Option<ParseCallback<T>>,
    Option<ParseCallback<T>>,
    Precedence,
);

impl<'a> Compiler<'a> {
    fn new(lexemes: &'a [Lexeme]) -> Self {
        Self {
            lexemes,
            current: 0,
            cunit: CUnit::Package(PackageUnit::new()),
            errs: Vec::new(),
            panic: false,
            scope: Scope::new(),
            control_flow: ControlFlow::new(),
            cur_package: None,
        }
    }

    fn compile(&mut self) -> CUnit {
        self.add_code(OpCode::Noop);
        self.decl_package();

        while !self.consume_if(Token::Eof) {
            self.decl();
        }

        self.add_entry_point();
        self.cunit.clone()
    }

    fn decl_package(&mut self) {
        if !self.is_package_scope() {
            self.err("Package declaration can be only in package level".to_string());
            return;
        }

        if self.consume_if(Token::Package) {
            let name = self.parse_var().to_string();
            if let CUnit::Package(p) = &mut self.cunit {
                p.set_name(name.clone());
                self.cur_package = Some(Package(name));
            } else {
                panic!("Unreachable code"); //FIXME
            }
        } else {
            self.err("Package declaration expected".to_string());
        }

        self.consume(Token::Semicolon);
    }

    fn decl(&mut self) {
        if self.consume_if(Token::Var) {
            self.decl_var();
        } else if self.consume_if(Token::Const) {
            self.decl_const();
        } else if self.consume_if(Token::Func) {
            self.decl_func();
        } else {
            self.stmt();
        }

        if self.panic {
            self.recover();
        }
    }

    fn decl_var(&mut self) {
        let name = self.parse_var().to_string();

        self.decl_scoped_var(name.clone());

        // FIXME change to just parse type and remake logic, same in const
        let val_type = self.parse_type_optionally(Token::Equal);

        if self.consume_if(Token::Equal) {
            self.expr();

            self.def_var(name, val_type, true);
        } else {
            if val_type.is_none() {
                self.err("Type declaration expected.".to_string());
            }

            self.add_code(OpCode::PutDefaultValue(val_type.clone().unwrap()));
            self.def_var(name, val_type, false);
        }

        self.consume(Token::Semicolon);
    }

    fn stmt_decl_short_var(&mut self) {
        self.expr_decl_short_var();
        self.consume(Token::Semicolon);
    }

    fn expr_decl_short_var(&mut self) {
        let name = self.parse_var().to_string();
        self.consume(Token::ColonEqual);
        self.decl_scoped_var(name.clone());
        self.expr();
        self.def_var(name, None, false);
    }

    fn def_var(&mut self, name: String, val_type: Option<ValType>, validate: bool) {
        if self.is_global_scope() {
            self.add_code(OpCode::VarGlobal(name, val_type));
        } else {
            if validate {
                if let Some(val_type) = val_type {
                    self.add_code(OpCode::ValidateType(val_type));
                }
            }
            self.scope.init_last();
        }
    }

    fn decl_const(&mut self) {
        let name = self.parse_var().to_string();

        self.decl_scoped_const(name.clone());

        let val_type = self.parse_type_optionally(Token::Equal);
        self.consume(Token::Equal);
        self.expr_const();

        if self.is_global_scope() {
            self.add_code(OpCode::ConstGlobal(name, val_type));
        } else {
            if let Some(val_type) = val_type {
                self.add_code(OpCode::ValidateType(val_type));
            }

            self.scope.init_last();
        }

        self.consume(Token::Semicolon);
    }

    fn decl_func(&mut self) {
        let name = self.parse_var().to_string();
        // FIXME we need this to be solved for inner nested functions
        // self.decl_scoped_var(name.clone());
        self.scope.init_last();

        self.func(Some(name.clone()));
        self.def_var(name, None, false);
    }

    fn func(&mut self, name: Option<String>) {
        let cunit = CUnit::Function(FuncUnit::new(name));
        let cunit = mem::replace(&mut self.cunit, cunit);

        self.begin_scope();
        self.consume(Token::LeftParen);

        let mut params = Vec::<Param>::new();
        if !self.check(Token::RightParen) {
            loop {
                if params.len() > FuncUnit::MAX_ARGC as usize {
                    self.err("Maximum parameter count reached.".to_string());
                }

                // FIXME add anon parameter support
                let param_name = String::from(self.parse_var());
                let param_type = self.parse_type();
                params.push(Param::new(param_name.clone(), param_type.clone()));

                // FIXME those two methods ought to be together, maybe group them?
                self.decl_scoped_var(param_name.clone());
                self.def_var(param_name, Some(param_type), false);

                if !self.consume_if(Token::Comma) {
                    break;
                }
            }
        }

        //FIXME rethink param type validation
        let len = params.len();
        if len >= 1 {
            for (i, param) in params.iter().enumerate() {
                self.add_code(OpCode::ValidateTypeAt(param.v_type().clone(), len - i - 1));
            }
        }

        self.consume(Token::RightParen);
        let ret_type = self.parse_type_optionally(Token::LeftCurlyBrace);

        self.consume(Token::LeftCurlyBrace);
        self.block_body();
        self.end_scope();

        let mut cunit = mem::replace(&mut self.cunit, cunit);
        if let CUnit::Function(funit) = &mut cunit {
            funit.params = params;
            funit.ret_type = ret_type;

            match EntryPoint::check_entry_point(self.cur_package.as_ref().unwrap(), funit) {
                Ok(()) => {}
                Err(e) => self.err(e.0),
            }
        } else {
            panic!("Compilation unit must be of function type");
        }

        self.add_code(OpCode::Func(cunit));
    }

    fn rule(&self, t: &Token) -> ParseRule<Self> {
        match t {
            Token::LeftParen => (Some(Self::group), Some(Self::call), Precedence::Call),
            Token::RightParen => (None, None, Precedence::None),
            Token::LeftCurlyBrace => (None, None, Precedence::None),
            Token::RightCurlyBrace => (None, None, Precedence::None),
            Token::Comma => (None, None, Precedence::None),
            Token::Dot => (None, None, Precedence::None),
            Token::Minus => (Some(Self::unary), Some(Self::binary), Precedence::Term),
            Token::Plus => (Some(Self::unary), Some(Self::binary), Precedence::Term),
            Token::Colon => (None, None, Precedence::None),
            Token::Semicolon => (None, None, Precedence::None),
            Token::Slash => (None, Some(Self::binary), Precedence::Factor),
            Token::Modulus => (None, Some(Self::binary), Precedence::Factor),
            Token::Asterisk => (None, Some(Self::binary), Precedence::Factor),
            Token::Bang => (Some(Self::unary), None, Precedence::None),
            Token::BangEqual => (None, Some(Self::binary), Precedence::Equality),
            Token::Equal => (None, None, Precedence::None),
            Token::EqualEqual => (None, Some(Self::binary), Precedence::Comparison),
            Token::Greater => (None, Some(Self::binary), Precedence::Comparison),
            Token::GreaterEqual => (None, Some(Self::binary), Precedence::Comparison),
            Token::Less => (None, Some(Self::binary), Precedence::Comparison),
            Token::LessEqual => (None, Some(Self::binary), Precedence::Comparison),
            Token::Identifier => (Some(Self::var), None, Precedence::None),
            Token::StringLiteral => (Some(Self::string), None, Precedence::None),
            Token::IntLiteral => (Some(Self::int), None, Precedence::None),
            Token::FloatLiteral => (Some(Self::float), None, Precedence::None),
            Token::LogicAnd => (None, Some(Self::and), Precedence::And),
            Token::LogicOr => (None, Some(Self::or), Precedence::Or),
            Token::Struct => (None, None, Precedence::None),
            Token::Else => (None, None, Precedence::None),
            Token::For => (None, None, Precedence::None),
            Token::Switch => (None, None, Precedence::None),
            Token::Case => (None, None, Precedence::None),
            Token::Func => (None, None, Precedence::None),
            Token::If => (None, None, Precedence::None),
            Token::False => (Some(Self::literal), None, Precedence::None),
            Token::True => (Some(Self::literal), None, Precedence::None),
            Token::Var => (None, None, Precedence::None),
            Token::Const => (None, None, Precedence::None),
            Token::Eof => (None, None, Precedence::None),
            // FIXME move to error
            tok => panic!("Unknown token {}", tok),
        }
    }

    fn current(&self) -> &Lexeme {
        &self.lexemes[self.current]
    }

    fn prev(&self) -> &Lexeme {
        if self.current == 0 {
            // FIXME move to error
            panic!("No tokens yet consumed.");
        }

        &self.lexemes[self.current - 1]
    }

    fn next(&self) -> &Lexeme {
        // if self.current + 1 <= self.lexemes.len()
        &self.lexemes[self.current + 1]
    }

    fn consume(&mut self, tok: Token) {
        if self.current().token == tok {
            self.advance();
        } else {
            self.err(format!(
                "Expected token \"{}\", got \"{}\"",
                tok,
                self.current().token
            ));
        }
    }

    fn consume_if(&mut self, tok: Token) -> bool {
        if !self.check(tok) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&mut self, tok: Token) -> bool {
        self.current().token == tok
    }

    fn check_next(&mut self, tok: Token) -> bool {
        self.next().token == tok
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn err(&mut self, msg: String) {
        self.panic = true;
        //FIXME: consider passing a handler
        eprintln!("\x1b[0;31m{} Error: {}\x1b[0m", self.current().pos, msg,);
        self.errs.push(msg);
    }

    fn recover(&mut self) {
        self.panic = false;
        use Token::*;
        while self.current().token != Eof {
            if self.prev().token == Semicolon {
                return;
            }
            match self.current().token {
                Struct | Func | Var | If | For | Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn stmt(&mut self) {
        self.err_if_package();

        // FIXME: change defer/debug
        if self.consume_if(Token::Defer) {
            self.expr();
            self.consume(Token::Semicolon);
            self.add_code(OpCode::Defer);
            // FIXME: defer/debug
        } else if self.consume_if(Token::For) {
            self.stmt_for();
        } else if self.consume_if(Token::Switch) {
            self.stmt_switch();
        } else if self.consume_if(Token::If) {
            self.stmt_if();
        } else if self.consume_if(Token::LeftCurlyBrace) {
            self.stmt_block();
        } else if self.consume_if(Token::Continue) {
            self.stmt_continue();
        } else if self.consume_if(Token::Break) {
            self.stmt_break();
        } else if self.consume_if(Token::Return) {
            self.stmt_return();
        } else if self.check(Token::Identifier) && self.check_next(Token::ColonEqual) {
            self.stmt_decl_short_var();
        } else {
            self.stmt_expr();
        }
    }

    fn stmt_block(&mut self) {
        self.begin_scope();
        self.block_body();
        self.end_scope();
    }

    fn stmt_continue(&mut self) {
        if !self.control_flow.is_continuable() {
            self.err("\"continue\" can be used in loops only".to_string());
            return;
        }

        self.add_code(OpCode::BackJump(
            self.code_len() - self.control_flow.continue_jump(),
        ));
    }

    fn stmt_break(&mut self) {
        if !self.control_flow.is_breakable() {
            self.err("\"break\" is misplaced".to_string());
            return;
        }

        let jump = self.add_code(OpCode::Jump(0));
        self.control_flow.add_break(jump);
    }

    fn stmt_fallthrough(&mut self) {
        if !self.control_flow.is_fallthroughable() {
            self.err("\"fallthrough\" can be used in switch statements only".to_string());
            return;
        }

        self.add_code(OpCode::Fallthrough);
    }

    fn stmt_return(&mut self) {
        //FIXME add package checking type
        if self.consume_if(Token::Semicolon) {
            self.add_code(OpCode::Return(true));
        } else {
            self.expr();
            self.consume(Token::Semicolon);
            self.add_code(OpCode::Return(false));
        }
    }

    fn begin_scope(&mut self) {
        self.scope.depth += 1;
    }

    fn begin_switch(&mut self) {
        self.control_flow.enter_switch();
    }

    fn begin_loop(&mut self) {
        self.begin_scope();
        self.control_flow.enter_loop();
    }

    fn end_loop(&mut self) {
        self.control_flow.leave_loop();
        self.end_scope();
    }

    fn end_switch(&mut self) {
        self.control_flow.leave_switch();
    }

    fn end_scope(&mut self) {
        self.scope.depth -= 1;

        let mut remove_indices = vec![];
        for i in (0..self.scope.vars.len()).rev() {
            if self.scope.vars[i].depth <= self.scope.depth as isize {
                break;
            }

            remove_indices.push(i);
        }

        for rem in remove_indices {
            self.add_code(OpCode::Pop);
            self.scope.vars.remove(rem);
        }
    }

    fn err_if_package(&mut self) {
        if self.is_package_scope() {
            self.err(format!("Unexpected token {}", self.current().token));
        }
    }

    fn is_global_scope(&self) -> bool {
        self.scope.depth == 0
    }

    fn block_body(&mut self) {
        while !self.check(Token::RightCurlyBrace) && !self.check(Token::Eof) {
            self.decl();
        }

        self.consume(Token::RightCurlyBrace);
    }

    fn stmt_expr(&mut self) {
        if !self.check(Token::Semicolon) {
            self.expr();
            self.consume(Token::Semicolon);
            self.add_code(OpCode::Pop);
        } else {
            self.stmt_empty();
        }
    }

    fn last_op_code_index(&self) -> usize {
        let len = self.code_len();
        if len == 0 {
            return 0; //09
        }

        len - 1
    }

    fn expr(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    // FIXME: make parse only const expressions
    fn expr_const(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    //FIXME var -> name
    fn parse_var(&mut self) -> &str {
        self.consume(Token::Identifier);
        &self.prev().literal
    }

    //FIXME change name from var to name
    fn decl_scoped_var(&mut self, name: String) {
        if self.is_global_scope() {
            return;
        }

        if self.scope.has_defined(&name) {
            self.err(format!("redeclared \"{}\" in this scope.", name));
        }

        self.scope.add_var(name);
    }

    fn decl_scoped_const(&mut self, name: String) {
        if self.is_global_scope() {
            return;
        }

        if self.scope.has_defined(&name) {
            self.err(format!("redeclared {} in this scope.", name));
        }

        self.scope.add_const(name);
    }

    fn parse_type_optionally(&mut self, if_not: Token) -> Option<ValType> {
        if !self.check(if_not) {
            Some(self.parse_type())
        } else {
            None
        }
    }

    fn parse_type(&mut self) -> ValType {
        let current = self.current();
        let val_type = match current.token {
            Token::Bool => ValType::Bool,

            Token::Int8 => ValType::Int8,
            Token::Int16 => ValType::Int16,
            Token::Int32 => ValType::Int32,
            Token::Rune => ValType::Int32,
            Token::Int64 => ValType::Int64,
            Token::Int => ValType::Int,

            Token::Uint8 => ValType::Uint8,
            Token::Byte => ValType::Uint8,
            Token::Uint16 => ValType::Uint16,
            Token::Uint32 => ValType::Uint32,
            Token::Uint64 => ValType::Uint64,
            Token::Uint => ValType::Uint,
            Token::Uintptr => ValType::Uintptr,

            Token::Float32 => ValType::Float32,
            Token::Float64 => ValType::Float64,

            Token::Complex64 => ValType::Complex64,
            Token::Complex128 => ValType::Complex128,

            Token::String => ValType::String,
            Token::Identifier => ValType::Struct(current.literal.clone()),
            tok => panic!("Type expected, got {}.", tok),
        };

        self.advance();

        val_type
    }

    fn stmt_empty(&mut self) {
        self.consume(Token::Semicolon);
    }

    fn stmt_switch(&mut self) {
        self.begin_switch();

        if self.check(Token::LeftCurlyBrace) {
            // switch {}
            self.add_code(OpCode::Bool(Value::Bool(true)));
        } else {
            // switch expr {}
            self.expr();
        }
        self.consume(Token::LeftCurlyBrace);
        self.add_code(OpCode::Switch);

        let mut case_jump: Option<usize> = None;
        let mut break_jumps = vec![];

        let mut default_case = false;
        let mut default_jump: Option<usize> = None;

        while self.check(Token::Case) || self.check(Token::Default) {
            if let Some(sw_jump) = case_jump {
                self.finish_jump(sw_jump);
            }

            // default ----
            if self.consume_if(Token::Default) {
                if default_case {
                    self.err("Multiple \"defaults\" in switch.".to_string());
                }
                default_case = true;
                case_jump = Some(self.add_code(OpCode::DefaultCaseJump(0)));
                default_jump = Some(self.last_op_code_index());
            } else {
                self.consume(Token::Case);
                self.expr();
                case_jump = Some(self.add_code(OpCode::CaseJump(0)));
            }

            self.case_block();

            let br_jump = self.add_code(OpCode::CaseBreakJump(0));
            break_jumps.push(br_jump);
        }

        if let Some(sw_jump) = case_jump {
            self.finish_jump(sw_jump);
        }

        self.consume(Token::RightCurlyBrace);

        if let Some(default_jump) = default_jump {
            self.add_code(OpCode::DefaultJump(self.code_len() - default_jump));
        }

        break_jumps.append(self.control_flow.switch_breaks());
        for break_jump in break_jumps {
            self.finish_jump(break_jump);
        }

        self.end_switch();
    }

    fn case_block(&mut self) {
        self.begin_scope();

        self.consume(Token::Colon);
        self.add_code(OpCode::DoCaseBreakJump);

        while !self.check(Token::Case)
            && !self.check(Token::Default)
            && !self.check(Token::RightCurlyBrace)
            && !self.check(Token::Fallthrough)
        {
            self.decl();
        }

        if self.consume_if(Token::Fallthrough) {
            self.stmt_fallthrough();
            self.consume(Token::Semicolon); //FIXME or move it in stmt?
        }
        // FIXME change error msg when fallthrough is not the last stmt

        self.end_scope();
    }

    fn stmt_for(&mut self) {
        self.begin_loop();

        let (for_like, mut exit_jump) = if self.check(Token::Semicolon) {
            // no init clause
            // for ; expr; expr {}
            self.consume(Token::Semicolon);
            let jump = self.last_op_code_index();

            (true, jump)
        } else if self.check(Token::LeftCurlyBrace) {
            // for {}
            let jump = self.last_op_code_index();
            self.add_code(OpCode::Bool(Value::Bool(true)));

            (false, jump)
        } else {
            let jump = self.last_op_code_index();

            let short_decl = if self.check(Token::Identifier) && self.check_next(Token::ColonEqual)
            {
                self.expr_decl_short_var();
                true
            } else {
                self.expr();
                false
            };

            if self.check(Token::Semicolon) {
                // for expr; expr; expr {}
                self.consume(Token::Semicolon);
                if !short_decl {
                    self.add_code(OpCode::Pop);
                }

                let jump = self.last_op_code_index();
                (true, jump)
            } else {
                // for expr {}
                (false, jump)
            }
        };

        if for_like {
            if self.consume_if(Token::Semicolon) {
                self.add_code(OpCode::Bool(Value::Bool(true)));
            } else {
                self.expr();
                self.consume(Token::Semicolon);
            }
        }

        let if_jump = self.add_code(OpCode::IfFalseJump(0));
        self.add_code(OpCode::Pop);

        if for_like && !self.check(Token::LeftCurlyBrace) {
            let inc_jump = self.add_code(OpCode::Jump(0));
            let inc_begin = self.last_op_code_index();

            self.control_flow.add_continue(exit_jump);

            self.expr();
            self.add_code(OpCode::Pop);

            self.add_code(OpCode::BackJump(self.code_len() - exit_jump));
            exit_jump = inc_begin;

            self.control_flow.add_continue(exit_jump);
            self.finish_jump(inc_jump);
        } else {
            self.control_flow.add_continue(exit_jump);
        }

        self.consume(Token::LeftCurlyBrace);
        self.stmt_block();

        let exit_jump = self.code_len() - exit_jump;
        self.control_flow.add_continue(exit_jump);
        self.add_code(OpCode::BackJump(exit_jump));

        self.finish_jump(if_jump);

        self.add_code(OpCode::Pop);

        let mut break_jumps = Vec::new();
        break_jumps.append(self.control_flow.loop_breaks());

        for inserted_break in break_jumps {
            self.finish_jump(inserted_break);
        }

        self.end_loop();
    }

    fn stmt_if(&mut self) {
        self.expr();
        let if_jump = self.add_code(OpCode::IfFalseJump(0));
        self.add_code(OpCode::Pop);

        self.consume(Token::LeftCurlyBrace);
        self.stmt_block();

        let jump = self.add_code(OpCode::Jump(0));
        self.finish_jump(if_jump);

        self.add_code(OpCode::Pop);
        if self.consume_if(Token::Else) {
            if self.consume_if(Token::If) {
                self.stmt_if();
            } else {
                self.consume(Token::LeftCurlyBrace);
                self.stmt_block();
            }
        }

        self.finish_jump(jump);
    }

    fn and(&mut self, _: bool) {
        let if_jump = self.add_code(OpCode::IfFalseJump(0));
        self.add_code(OpCode::Pop);
        self.parse_precedence(Precedence::And);
        self.finish_jump(if_jump);
    }

    fn or(&mut self, _: bool) {
        let if_jump = self.add_code(OpCode::IfFalseJump(0));
        let jump = self.add_code(OpCode::Jump(0));

        self.finish_jump(if_jump);
        self.add_code(OpCode::Pop);

        self.parse_precedence(Precedence::Or);
        self.finish_jump(jump);
    }

    fn string(&mut self, _: bool) {
        let string = Value::String(self.prev().literal.clone());
        self.add_code(OpCode::String(string));
    }

    fn int(&mut self, _: bool) {
        let int = Value::Int(self.prev().literal.parse::<isize>().unwrap());
        self.add_code(OpCode::IntLiteral(int));
    }

    fn float(&mut self, _: bool) {
        let float = Value::Float64(self.prev().literal.parse::<f64>().unwrap());
        self.add_code(OpCode::FloatLiteral(float));
    }

    fn var(&mut self, assign: bool) {
        self.named_var(assign);
    }

    // FIXME: refactor the logic
    fn named_var(&mut self, assign: bool) {
        let name = self.prev().literal.clone();
        let resolved = self.scope.resolve(&name);

        if assign && self.consume_if(Token::Equal) {
            self.expr();

            let code = if let Some((i, mutable)) = resolved {
                if mutable {
                    OpCode::SetLocal(i)
                } else {
                    self.err("Trying to assign to a const".to_string());
                    return;
                }
            } else {
                OpCode::SetGlobal(name)
            };

            self.add_code(code)
        } else {
            let code = if let Some((i, _)) = resolved {
                if self.scope.vars[i].depth == -1 {
                    OpCode::GetGlobal(name)
                } else {
                    OpCode::GetLocal(i)
                }
            } else {
                OpCode::GetGlobal(name)
            };

            self.add_code(code)
        };
    }

    fn group(&mut self, _: bool) {
        self.expr();
        self.consume(Token::RightParen);
    }

    fn unary(&mut self, _: bool) {
        let operator = self.prev().token;
        self.parse_precedence(Precedence::Unary);

        let code = match operator {
            Token::Bang => OpCode::Not,
            Token::Minus => OpCode::Negate,
            Token::Plus => OpCode::PlusNoop,
            _ => {
                return;
            }
        };

        self.add_code(code);
    }

    fn parse_precedence(&mut self, prec: Precedence) {
        self.advance();
        let prefix = self.rule(&self.prev().token).0;
        if prefix.is_none() {
            //FIXME
            return;
        }

        let can_assign = prec <= Precedence::Assignment;
        prefix.unwrap()(self, can_assign);

        while prec <= self.rule(&self.current().token).2 {
            self.advance();
            let inflix = self.rule(&self.prev().token).1;
            if inflix.is_none() {
                //FIXME
                return;
            }

            inflix.unwrap()(self, can_assign);
        }
    }

    fn parse_args(&mut self) -> u8 {
        let mut argc = 0;
        if !self.check(Token::RightParen) {
            loop {
                self.expr();
                if argc == FuncUnit::MAX_ARGC {
                    self.err("Too many args".to_string());
                    break; //FIXME check this
                }

                argc += 1;
                if !self.consume_if(Token::Comma) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen);

        argc
    }

    fn binary(&mut self, _assign: bool) {
        let operator = self.prev().token;
        let precedence = self.rule(&operator).2;

        self.parse_precedence(precedence.next());

        let code = match operator {
            Token::Plus => OpCode::Add,
            Token::Minus => OpCode::Subtract,
            Token::Asterisk => OpCode::Multiply,
            Token::Slash => OpCode::Divide,
            Token::Modulus => OpCode::Remainder,
            Token::BangEqual => OpCode::NotEqual,
            Token::EqualEqual => OpCode::Equal,
            Token::Greater => OpCode::Greater,
            Token::GreaterEqual => OpCode::GreaterEqual,
            Token::Less => OpCode::Less,
            Token::LessEqual => OpCode::LessEqual,
            _ => {
                return;
            }
        };

        self.add_code(code);
    }

    fn literal(&mut self, _: bool) {
        let code = match self.prev().token {
            Token::True => OpCode::Bool(Value::Bool(true)),
            Token::False => OpCode::Bool(Value::Bool(false)),
            _ => {
                return;
            }
        };

        self.add_code(code);
    }

    fn call(&mut self, _: bool) {
        let args = self.parse_args();
        self.add_code(OpCode::Call(args));
    }

    fn add_code(&mut self, code: OpCode) -> usize {
        let pos = if self.current > 0 {
            self.prev().pos
        } else {
            self.current().pos
        };

        self.cunit.chunk_mut().write(code, pos)
    }

    fn add_entry_point(&mut self) {
        self.add_code(OpCode::GetGlobal(EntryPoint::FUNC_NAME.to_string()));
        self.add_code(OpCode::Call(0));
    }

    fn finish_jump(&mut self, i: usize) {
        let jump = self.last_op_code_index() - i;

        use OpCode::*;
        let jump = match &self.cunit.chunk().codes()[i] {
            Jump(_) => Jump(jump),
            CaseJump(_) => CaseJump(jump),
            DefaultCaseJump(_) => DefaultCaseJump(jump),
            DefaultJump(_) => DefaultJump(jump),
            CaseBreakJump(_) => CaseBreakJump(jump),
            BackJump(_) => BackJump(jump),
            IfFalseJump(_) => IfFalseJump(jump),
            code => panic!("Cannot change non-jump opcode \"{:?}\".", code),
        };

        self.cunit.chunk_mut().write_at(i, jump);
    }

    fn code_len(&self) -> usize {
        self.cunit.chunk().codes().len()
    }

    fn is_package_scope(&self) -> bool {
        match self.cunit {
            CUnit::Package(_) => true,
            CUnit::Function(_) => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
enum Precedence {
    None = 0,
    Assignment,
    Or,
    And,
    Equality,
    Comparison,
    Term,
    Factor,
    Unary,
    Call,
    Primary,
}

impl Precedence {
    fn next(&self) -> Self {
        match self {
            Self::None => Self::Assignment,
            Self::Assignment => Self::Or,
            Self::Or => Self::And,
            Self::And => Self::Equality,
            Self::Equality => Self::Comparison,
            Self::Comparison => Self::Term,
            Self::Term => Self::Factor,
            Self::Factor => Self::Unary,
            Self::Unary => Self::Call,
            Self::Call => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}

struct Package(String);
struct EntryPoint;
struct SignError(String);
type SignValidationResult = std::result::Result<(), SignError>;

impl EntryPoint {
    const PACK_NAME: &'static str = "main";
    const FUNC_NAME: &'static str = "main";

    fn check_entry_point(pack: &Package, funit: &FuncUnit) -> SignValidationResult {
        if pack.0 == Self::PACK_NAME && funit.name == Self::FUNC_NAME {
            if Self::validate_signature(funit) {
                Ok(())
            } else {
                Err(SignError(format!(
                    "Function \"{}\" must not have parameters and a return value",
                    Self::FUNC_NAME,
                )))
            }
        } else {
            Ok(())
        }
    }

    fn validate_signature(funit: &FuncUnit) -> bool {
        funit.ret_type.is_none() && funit.params.is_empty()
    }
}
