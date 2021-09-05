use std::mem;

use self::error::CompileError;
pub use self::error::{ErrorHandler, ToStderrErrorHandler};
use self::flow::ControlFlow;
pub(crate) use self::opcode::OpCode;
use self::scope::Scope;
use self::unit::{CompilationUnit as CUnit, FuncUnit, PackageUnit};
pub(crate) use self::value::{RefIterator, TypeError, Value};
use self::vtype::FuncType;
pub(crate) use self::vtype::ValType;
use crate::lex::lexeme::{Lexeme, Token};
use crate::lex::Lexer;
use crate::vm::CUnitFrame;

pub(crate) mod error;
mod flow;
mod opcode;
mod scope;
pub(crate) mod unit;
mod value;
mod vtype;

pub fn compile(src: &str, err_handler: &mut dyn ErrorHandler) -> CUnitFrame {
    let mut lexer = Lexer::new(src);
    let (lexemes, errors) = lexer.lex();

    if !errors.is_empty() {
        err_handler.on_error(errors);
    }

    let mut parser = Compiler::new(lexemes);
    let (cunit, errors) = parser.compile();

    if !errors.is_empty() {
        err_handler.on_error(errors);
    }

    CUnitFrame::new(cunit)
}

struct Compiler<'a> {
    lexemes: &'a [Lexeme],
    current: usize,
    cunit: CUnit,
    errs: Vec<Box<dyn std::error::Error>>,
    panic: bool,
    scope: Scope,
    control_flow: ControlFlow,
    cur_package: Option<Package>,
    index_depth: usize,
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
            index_depth: 0,
        }
    }

    /// Compilation entry point
    /// Returns Compilation Unit ("main" package) and a slice of errors
    fn compile(&mut self) -> (CUnit, &[Box<dyn std::error::Error>]) {
        self.add_code(OpCode::Noop);
        self.decl_package();

        while !self.consume_if(Token::Eof) {
            self.decl();
        }

        self.add_entry_point();
        (self.cunit.clone(), &self.errs)
    }

    /// Parses package declaration statement
    fn decl_package(&mut self) {
        if !self.is_package_scope() {
            self.err("Package declaration can be only in package level".to_string());
            return;
        }

        if self.consume_if(Token::Package) {
            let name = self.parse_name().to_string();
            if let CUnit::Package(p) = &mut self.cunit {
                p.set_name(name.clone());
                self.cur_package = Some(Package(name));
            } else {
                panic!("Compiling did not start with a package");
            }
        } else {
            self.err("Package declaration expected".to_string());
        }

        self.consume(Token::Semicolon);
    }

    /// Various types of declarations (including group declarations)
    fn decl(&mut self) {
        if self.consume_if(Token::Var) {
            self.decl_group_var();
        } else if self.consume_if(Token::Const) {
            self.decl_group_const();
        } else if self.consume_if(Token::Func) {
            self.decl_func();
        } else {
            self.stmt();
        }

        if self.panic {
            self.recover();
        }
    }

    fn decl_group_var(&mut self) {
        if self.consume_if(Token::LeftParen) {
            while !self.consume_if(Token::RightParen) {
                self.decl_var();
            }
            self.consume(Token::Semicolon);
        } else {
            self.decl_var();
        }
    }

    fn decl_var(&mut self) {
        let name = self.parse_name().to_string();

        self.decl_scoped_name(name.clone());

        let vtype = if !self.check(Token::Equal) {
            Some(self.parse_type())
        } else {
            None
        };

        if self.consume_if(Token::Equal) {
            self.expr();
            self.def_var(name, vtype, true, true);
        } else {
            if vtype.is_none() {
                self.err("Type declaration expected.".to_string());
            }

            self.add_code(OpCode::PutDefaultValue(vtype.clone().unwrap()));
            self.def_var(name, vtype, false, true);
        }

        self.consume(Token::Semicolon);
    }

    fn expr_decl_short_var(&mut self) {
        let name = self.parse_name().to_string();
        self.consume(Token::ColonEqual);
        self.decl_scoped_name(name.clone());
        self.expr();
        self.def_var(name, None, false, true);
    }

    //FIXME change flags
    fn def_var(&mut self, name: String, val_type: Option<ValType>, validate: bool, litcast: bool) {
        if self.is_global_scope() {
            self.add_code(OpCode::VarGlobal(name, val_type));
        } else {
            //FIXME change logic
            if validate {
                if let Some(val_type) = val_type {
                    self.add_code(OpCode::ValidateTypeWithLiteralCast(val_type));
                } else if litcast {
                    self.add_code(OpCode::LiteralCast);
                }
            } else if litcast {
                self.add_code(OpCode::LoseSoftReference);
                self.add_code(OpCode::LiteralCast);
            }

            self.scope.init_last();
        }
    }

    fn decl_group_const(&mut self) {
        if self.consume_if(Token::LeftParen) {
            while !self.consume_if(Token::RightParen) {
                self.decl_const();
            }
            self.consume(Token::Semicolon);
        } else {
            self.decl_const();
        }
    }

    fn decl_const(&mut self) {
        let name = self.parse_name().to_string();

        self.decl_scoped_const(name.clone());

        let vtype = if !self.check(Token::Equal) {
            Some(self.parse_type())
        } else {
            None
        };
        self.consume(Token::Equal);
        self.expr_const();

        if self.is_global_scope() {
            self.add_code(OpCode::ConstGlobal(name, vtype));
        } else {
            if let Some(vtype) = vtype {
                self.add_code(OpCode::ValidateTypeWithLiteralCast(vtype));
            } else {
                self.add_code(OpCode::LiteralCast);
            }

            self.scope.init_last();
        }

        self.consume(Token::Semicolon);
    }

    fn decl_func(&mut self) {
        let name = self.parse_name().to_string();
        let ftype = self.func(Some(name.clone()));
        self.def_var(name, Some(ValType::Func(Box::new(ftype))), false, false);
    }

    fn func(&mut self, name: Option<String>) -> FuncType {
        self.begin_scope();
        self.consume(Token::LeftParen);

        let mut param_names = Vec::<String>::new();
        let mut param_types = Vec::<ValType>::new();

        if !self.check(Token::RightParen) {
            loop {
                if param_names.len() > FuncUnit::MAX_ARGC as usize {
                    self.err("Maximum parameter count reached.".to_string());
                }

                // no anonimous parameter support
                let param_name = String::from(self.parse_name());
                let param_type = self.parse_type();

                param_names.push(param_name.clone());
                param_types.push(param_type.clone());

                self.decl_scoped_name(param_name.clone());
                // It neither is a global scope nor validation case, so no codes are added here
                self.def_var(param_name, Some(param_type), false, false);

                if !self.consume_if(Token::Comma) {
                    break;
                }

                if self.check(Token::RightParen) {
                    break;
                }
            }
        }

        self.consume(Token::RightParen);

        let ret_type = if !self.check(Token::LeftCurlyBrace) {
            Some(self.parse_type())
        } else {
            None
        };
        let ftype = FuncType::new(param_types.clone(), ret_type);

        // Before this line there ought not to be any OpCode addition to the current CUnit
        let cunit = CUnit::Function(FuncUnit::new(name, ftype.clone(), param_names));
        let cunit = mem::replace(&mut self.cunit, cunit);

        //FIXME rethink param type validation
        let len = param_types.len();
        if len >= 1 {
            for (i, param_type) in param_types.iter().enumerate() {
                self.add_code(OpCode::ValidateTypeAtWithLiteralCast(
                    param_type.clone(),
                    len - i - 1,
                ));
            }
        }

        self.consume(Token::LeftCurlyBrace);
        self.block_body();
        self.end_scope();

        let mut cunit = mem::replace(&mut self.cunit, cunit);
        if let CUnit::Function(funit) = &mut cunit {
            match entry_point::check(self.cur_package.as_ref().unwrap(), funit) {
                Ok(()) => {}
                Err(e) => self.err(e.0),
            }
        } else {
            panic!("Compilation unit must be of function type");
        }

        self.add_code(OpCode::Func(cunit));

        ftype
    }

    fn rule(&self, mut t: &Token) -> ParseRule<Self> {
        if matches!(
            t,
            Token::Int
                | Token::Int8
                | Token::Int16
                | Token::Int32
                | Token::Int64
                | Token::Uint
                | Token::Uint8
                | Token::Uint16
                | Token::Uint32
                | Token::Uint64
                | Token::Uintptr
                | Token::Float32
                | Token::Float64
                | Token::String
        ) {
            t = &Token::Identifier
        };

        match t {
            Token::LeftParen => (Some(Self::group), Some(Self::call), Precedence::Call),
            Token::RightParen => (None, None, Precedence::None),
            Token::LeftCurlyBrace => (None, None, Precedence::None),
            Token::RightCurlyBrace => (None, None, Precedence::None),
            Token::LeftBracket => (Some(Self::literal), Some(Self::index), Precedence::Index),
            Token::RightBracket => (None, None, Precedence::None),
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
            tok => panic!("Unknown token {}", tok),
        }
    }

    fn current(&self) -> &Lexeme {
        &self.lexemes[self.current]
    }

    fn prev(&self) -> &Lexeme {
        &self.lexemes[self.current - 1]
    }

    fn next(&self) -> &Lexeme {
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

    fn check_in(&mut self, toks: &[Token]) -> bool {
        toks.contains(&self.current().token)
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn rollback(&mut self) {
        self.current -= 1;
    }

    fn err(&mut self, msg: String) {
        self.panic = true;
        self.errs
            .push(Box::new(CompileError(msg, self.current().pos)));
    }

    fn recover(&mut self) {
        self.panic = false;
        use Token::*;
        while self.current().token != Eof {
            if self.prev().token == Semicolon {
                return;
            }
            match self.current().token {
                Struct | Func | Var | If | For | Return | Switch | Const => return,
                _ => {}
            }

            self.advance();
        }
    }

    fn stmt(&mut self) {
        self.err_if_package();
        if self.consume_if(Token::For) {
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
        } else {
            self.stmt_simple();
        }
    }

    fn stmt_block(&mut self) {
        self.begin_scope();
        self.block_body();
        self.end_scope();
    }

    /// Simple expression or an empty expression with a semicolon
    fn stmt_simple(&mut self) {
        self.expr_simple();
        self.consume(Token::Semicolon);
    }

    fn expr_simple(&mut self) {
        if self.check(Token::Identifier) && self.check_next(Token::ColonEqual) {
            self.expr_decl_short_var();
        } else {
            self.expr_expr();
        };
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
        self.consume_if(Token::Semicolon);
    }

    fn expr_expr(&mut self) {
        if !self.check(Token::Semicolon) {
            self.expr();
            self.add_code(OpCode::Pop);
        }
    }

    fn last_op_code_index(&self) -> usize {
        let len = self.code_len();
        if len == 0 {
            return 0; // returns 0 if there are no opcodes
        }

        len - 1
    }

    fn expr(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn expr_const(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_name(&mut self) -> &str {
        self.consume(Token::Identifier);
        &self.prev().literal
    }

    fn decl_scoped_name(&mut self, name: String) {
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

    /// Either returns a fully constructed value type or panics
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

            Token::LeftBracket => {
                self.advance();

                // slice
                if self.check(Token::RightBracket) {
                    self.advance();
                    let slice_type = self.parse_type();

                    return ValType::Slice(Box::new(slice_type));
                }

                // array
                let size = self.parse_constant_int();
                self.consume(Token::RightBracket);
                let array_type = self.parse_type();

                return ValType::Array(Box::new(array_type), size);
            }

            Token::Identifier => ValType::Struct(current.literal.clone()),
            tok => panic!("Type expected, got {}.", tok),
        };

        self.advance();

        val_type
    }

    fn parse_constant_int(&mut self) -> usize {
        self.advance();
        let size = self.prev().literal.parse::<usize>().unwrap();

        size
    }

    /// For array or slice literals (the curly braced part of expressions like `[2]int{1, 2}`)
    fn parse_array_body(&mut self) -> usize {
        self.consume(Token::LeftCurlyBrace);

        let mut len = 0;
        if !self.check(Token::RightCurlyBrace) {
            loop {
                self.expr();
                len += 1;

                if !self.consume_if(Token::Comma) {
                    break;
                }

                if self.check(Token::RightCurlyBrace) {
                    break;
                }
            }
        }
        self.consume(Token::RightCurlyBrace);

        len
    }

    /// All forms of `switch` statements
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
        self.consume_if(Token::Semicolon);

        if let Some(default_jump) = default_jump {
            self.add_code(OpCode::DefaultJump(self.code_len() - default_jump));
        }

        break_jumps.append(self.control_flow.switch_breaks());
        for break_jump in break_jumps {
            self.finish_jump(break_jump);
        }

        self.end_switch();
    }

    /// Case blocks of switch statements
    fn case_block(&mut self) {
        // each case block has its own scope
        // even tho its not enclosed in curly braces
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

        // `fallthrough`, if present, must be the last statement
        if self.consume_if(Token::Fallthrough) {
            self.stmt_fallthrough();
            self.consume(Token::Semicolon);
        }
        // FIXME change error msg when fallthrough is not the last stmt

        self.end_scope();
    }

    /// All forms of `for` statements
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
            self.expr_simple();

            if self.check(Token::Semicolon) {
                // for expr; expr; expr {}
                self.consume(Token::Semicolon);
                let jump = self.last_op_code_index();
                (true, jump)
            } else {
                // for expr {}
                self.pop_code(OpCode::Pop);
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

            self.expr_simple();
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

    /// `if` statement, covers `if else` and `else` clauses as well
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
        let int = Value::IntLiteral(self.prev().literal.parse::<isize>().unwrap());
        self.add_code(OpCode::IntLiteral(int));
    }

    fn float(&mut self, _: bool) {
        let float = Value::FloatLiteral(self.prev().literal.parse::<f64>().unwrap());
        self.add_code(OpCode::FloatLiteral(float));
    }

    fn var(&mut self, assign: bool) {
        self.named_var(if assign {
            val_context::ASSIGNMENT
        } else {
            val_context::NONE
        });
    }

    /// Expressions with a compound operator like `+=`
    fn expr_compound_assign(&mut self, context: val_context::Context) {
        let name = self.prev().literal.clone();
        let resolved = self.scope.resolve(&name);

        let (get_code, set_code) = if let Some((i, mutable)) = resolved {
            if mutable {
                if val_context::is_index(context) {
                    (OpCode::GetIndex, OpCode::SetIndex)
                } else {
                    (OpCode::GetLocal(i), OpCode::SetLocal(i))
                }
            } else {
                self.err("Trying to assign to a const".to_string());
                return;
            }
        } else if val_context::is_index(context) {
            (OpCode::GetIndex, OpCode::SetIndex)
        } else {
            (OpCode::GetGlobal(name.clone()), OpCode::SetGlobal(name))
        };

        // Hack to make possible nested index call in an assignment context
        if val_context::is_index(context) {
            self.duplicate_codes(self.index_depth * 2);
            self.index_depth = 0;
        }

        self.add_code(get_code);

        self.advance();
        let operator = self.prev().token;
        //FIXME add bitwise
        let code = match operator {
            Token::PlusEqual => OpCode::Add,
            Token::MinusEqual => OpCode::Subtract,
            Token::AsteriskEqual => OpCode::Multiply,
            Token::SlashEqual => OpCode::Divide,
            Token::ModulusEqual => OpCode::Remainder,
            _ => {
                return;
            }
        };

        self.expr();
        self.add_code(code);
        self.add_code(set_code);
    }

    /// Expressions with a unary assignment operator `++` or `--`
    fn expr_inc(&mut self, context: val_context::Context) {
        let name = self.prev().literal.clone();
        let resolved = self.scope.resolve(&name);

        let (get_code, set_code) = if let Some((i, mutable)) = resolved {
            if mutable {
                if val_context::is_index(context) {
                    (OpCode::GetIndex, OpCode::SetIndex)
                } else {
                    (OpCode::GetLocal(i), OpCode::SetLocal(i))
                }
            } else {
                self.err("Trying to assign to a const".to_string());
                return;
            }
        } else if val_context::is_index(context) {
            (OpCode::GetIndex, OpCode::SetIndex)
        } else {
            (OpCode::GetGlobal(name.clone()), OpCode::SetGlobal(name))
        };

        // Hack to make possible nested index call in an assignment context
        if val_context::is_index(context) {
            self.duplicate_codes(self.index_depth * 2);
            self.index_depth = 0;
        }

        self.add_code(get_code);
        self.advance();
        let code = match self.prev().token {
            Token::Inc => OpCode::Add,
            Token::Dec => OpCode::Subtract,
            _ => {
                return;
            }
        };

        self.add_code(OpCode::IntLiteral(Value::IntLiteral(1)));
        self.add_code(code);
        self.add_code(set_code);
    }

    /// Assignment expression with a single assignment
    fn expr_assign(&mut self, context: val_context::Context) {
        let name = self.prev().literal.clone();
        let resolved = self.scope.resolve(&name);

        self.consume(Token::Equal);
        self.expr();
        let code = if let Some((i, mutable)) = resolved {
            if mutable {
                if val_context::is_index(context) {
                    OpCode::SetIndex
                } else {
                    OpCode::SetLocal(i)
                }
            } else {
                self.err("Trying to assign to a const".to_string());
                return;
            }
        } else if val_context::is_index(context) {
            OpCode::SetIndex
        } else {
            OpCode::SetGlobal(name)
        };

        self.add_code(code);
    }

    /// Parses named variable value.
    /// Expects `context` of a variable to be able to decide which opcodes to emit
    fn named_var(&mut self, context: val_context::Context) {
        if val_context::is_assignment(context) {
            if self.check(Token::Equal) {
                self.expr_assign(context);
            } else if self.check_in(&ASSIGN_OPERATORS) {
                self.expr_compound_assign(context);
            } else if self.check_in(&INC_OPERATORS) {
                self.expr_inc(context);
            } else {
                self.expr_get_var(context);
            }
        } else {
            self.expr_get_var(context);
        }
    }

    fn expr_get_var(&mut self, context: val_context::Context) {
        let name = self.prev().literal.clone();
        let resolved = self.scope.resolve(&name);

        let code = if let Some((i, _)) = resolved {
            if self.scope.vars[i].depth == -1 {
                if val_context::is_index(context) {
                    OpCode::GetIndex
                } else {
                    OpCode::GetGlobal(name)
                }
            } else if val_context::is_index(context) {
                OpCode::GetIndex
            } else {
                OpCode::GetLocal(i)
            }
        } else if val_context::is_index(context) {
            OpCode::GetIndex
        } else {
            OpCode::GetGlobal(name)
        };

        self.add_code(code);
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
            return;
        }

        let can_assign = prec <= Precedence::Assignment;
        prefix.unwrap()(self, can_assign);

        while prec <= self.rule(&self.current().token).2 {
            self.advance();
            let inflix = self.rule(&self.prev().token).1;
            if inflix.is_none() {
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

                if self.check(Token::RightParen) {
                    break;
                }
            }
        }
        self.consume(Token::RightParen);

        argc
    }

    fn binary(&mut self, _: bool) {
        let operator = self.prev().token;
        let precedence = self.rule(&operator).2;

        self.parse_precedence(precedence.next());

        // FIXME add bitwise
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
            Token::LeftBracket => {
                let code = if self.check(Token::RightBracket) {
                    // slice
                    OpCode::SliceLiteral
                } else {
                    // array
                    OpCode::ArrayLiteral
                };

                self.rollback();
                let vtype = self.parse_type();
                let len = self.parse_array_body();
                code(len, vtype)
            }
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

    fn index(&mut self, assign: bool) {
        let context = if assign {
            val_context::ASSIGNMENT | val_context::INDEX
        } else {
            val_context::INDEX
        };

        // FIXME revisit this nested index counting
        if val_context::is_assignment(context) {
            self.index_depth += 1;
        }

        self.expr();
        self.consume(Token::RightBracket);

        self.named_var(context);
        self.index_depth = 0;
    }

    fn add_code(&mut self, code: OpCode) -> usize {
        let pos = if self.current > 0 {
            self.prev().pos
        } else {
            self.current().pos
        };

        self.cunit.chunk_mut().write(code, pos)
    }

    /// Duplicates last N opcodes.
    fn duplicate_codes(&mut self, last: usize) {
        let len = self.cunit.chunk().codes().len();
        let mut codes = vec![];
        for i in 0..last {
            let code = &self.cunit.chunk().codes()[len - i - 1];
            codes.push(code.clone())
        }
        codes.reverse();
        for code in codes {
            self.add_code(code);
        }
    }

    /// Removes last OpCode if it matches.
    /// Panic otherwise to prevent possible errors
    fn pop_code(&mut self, code: OpCode) {
        let popped = self.cunit.chunk_mut().pop().unwrap();
        if mem::discriminant(&popped) != mem::discriminant(&code) {
            panic!("Wrong op code popped from a chunk");
        }
    }

    fn add_entry_point(&mut self) {
        self.add_code(OpCode::GetGlobal(entry_point::FUNC_NAME.to_string()));
        self.add_code(OpCode::Call(0));
    }

    /// Adjust the value of previously put Jump opcode
    /// `i` is the new position where the Jump must lead
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

/// Operation precedence
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
    Index,
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
            Self::Call => Self::Index,
            Self::Index => Self::Primary,
            Self::Primary => Self::Primary,
        }
    }
}

struct Package(String);

struct SignatureError(String);
type SignValidationResult = std::result::Result<(), SignatureError>;

/// Entry point validation and check.
mod entry_point {
    use super::*;

    pub(super) const FUNC_NAME: &str = "main";
    const PACK_NAME: &str = "main";

    pub(super) fn check(pack: &Package, funit: &FuncUnit) -> SignValidationResult {
        if pack.0 == PACK_NAME && funit.name() == FUNC_NAME {
            if validate_signature(funit) {
                Ok(())
            } else {
                Err(SignatureError(format!(
                    "Function \"{}\" must not have parameters and a return value",
                    FUNC_NAME,
                )))
            }
        } else {
            Ok(())
        }
    }

    fn validate_signature(funit: &FuncUnit) -> bool {
        funit.ret_type().is_none() && funit.param_names().is_empty()
    }
}

const ASSIGN_OPERATORS: [Token; 12] = [
    Token::PlusEqual,
    Token::MinusEqual,
    Token::AsteriskEqual,
    Token::SlashEqual,
    Token::ModulusEqual,
    Token::ModulusEqual,
    Token::BitwiseAndEqual,
    Token::BitwiseOrEqual,
    Token::BitwiseXorEqual,
    Token::BitClearEqual,
    Token::LeftShiftEqual,
    Token::RightShiftEqual,
];

const INC_OPERATORS: [Token; 2] = [Token::Inc, Token::Dec];

/// Value context.
mod val_context {
    pub type Context = u8;

    pub const NONE: u8 = 0x00;
    pub const ASSIGNMENT: u8 = 0x01;
    pub const INDEX: u8 = 0x02;

    pub fn is_assignment(context: Context) -> bool {
        context & ASSIGNMENT == ASSIGNMENT
    }

    pub fn is_index(context: Context) -> bool {
        context & INDEX == INDEX
    }
}
