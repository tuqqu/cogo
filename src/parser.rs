use crate::chunk::{Chunk, OpCode};
use crate::lexer::lexeme::{Lexeme, Token};
use crate::value::{ValType, Value};

pub struct Parser<'a> {
    lexemes: &'a [Lexeme],
    current: usize,
    chunk: Chunk,
    errs: Vec<String>,
    panic: bool,
    scope: Scope,
}

type ParseCallback<T> = fn(&mut T, bool);
type ParseRule<T> = (
    Option<ParseCallback<T>>,
    Option<ParseCallback<T>>,
    Precedence,
);

impl<'a> Parser<'a> {
    pub fn new(lexemes: &'a [Lexeme]) -> Self {
        Self {
            lexemes,
            current: 0,
            chunk: Chunk::new(),
            errs: Vec::new(),
            panic: false,
            scope: Scope::new(),
        }
    }

    pub fn parse(&mut self) -> Chunk {
        while !self.check_current(Token::Eof) {
            self.decl();
        }

        self.chunk.clone()
    }

    fn decl(&mut self) {
        if self.check_current(Token::Var) {
            self.var_decl();
        } else {
            self.stmt();
        }

        if self.panic {
            self.recover();
        }
    }

    fn var_decl(&mut self) {
        let name = self.parse_var("Expected var name".to_string()).to_string();

        self.decl_scoped_var(name.clone());

        let val_type = self.parse_type();

        if self.check_current(Token::Equal) {
            self.expr();

            if self.is_global_scope() {
                self.add_code(OpCode::VarGlobal(name, val_type));
            } else {
                if let Some(val_type) = val_type {
                    self.add_code(OpCode::ValidateType(val_type));
                }

                self.scope.init_last();
            }
        } else {
            if val_type.is_none() {
                self.err("Type declaration expected.".to_string());
            }

            if self.is_global_scope() {
                self.add_code(OpCode::VarGlobalNoInit(name, val_type.unwrap()));
            } else {
                if let Some(val_type) = val_type {
                    self.add_code(OpCode::PutDefaultValue(val_type));
                }
                self.scope.init_last();
            }
        }

        self.consume(Token::Semicolon, "Expected semicolon.".to_string());
    }

    fn rule(&self, t: &Token) -> ParseRule<Self> {
        match t {
            Token::LeftParen => (Some(Self::group), None, Precedence::None),
            Token::RightParen => (None, None, Precedence::None),
            Token::RightCurlyBrace => (None, None, Precedence::None),
            Token::Comma => (None, None, Precedence::None),
            Token::Dot => (None, None, Precedence::None),
            Token::Minus => (Some(Self::unary), Some(Self::binary), Precedence::Term),
            Token::Plus => (Some(Self::unary), Some(Self::binary), Precedence::Term),
            Token::Semicolon => (None, None, Precedence::None),
            Token::Slash => (None, Some(Self::binary), Precedence::Factor),
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
            Token::LogicAnd => (None, None, Precedence::None),
            Token::Struct => (None, None, Precedence::None),
            Token::Else => (None, None, Precedence::None),
            Token::For => (None, None, Precedence::None),
            Token::Func => (None, None, Precedence::None),
            Token::If => (None, None, Precedence::None),
            // Token::Nil => (Some(Self::unary), None, Precedence::None),
            Token::False => (Some(Self::literal), None, Precedence::None),
            Token::True => (Some(Self::literal), None, Precedence::None),
            Token::Var => (None, None, Precedence::None),
            Token::Eof => (None, None, Precedence::None),
            _ => panic!("Unknown token"),
        }
    }

    fn current(&self) -> &Lexeme {
        &self.lexemes[self.current]
    }

    fn prev(&self) -> &Lexeme {
        &self.lexemes[self.current - 1]
    }

    fn consume(&mut self, tok: Token, msg: String) {
        if self.current().token == tok {
            self.advance();
        } else {
            self.err(msg);
        }
    }

    fn check_current(&mut self, tok: Token) -> bool {
        if !self.check(tok) {
            return false;
        }

        self.advance();
        true
    }

    fn check(&mut self, tok: Token) -> bool {
        self.current().token == tok
    }

    fn advance(&mut self) {
        self.current += 1;
    }

    fn err(&mut self, msg: String) {
        self.panic = true;
        //FIXME: consider passing a handler
        eprintln!(
            "\x1b[0;31m error {:#?} {:?} {:#?}\x1b[0m",
            msg,
            self.current(),
            self.current().pos
        );
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
        // FIXME: change defer/debug
        if self.check_current(Token::Defer) {
            self.expr();
            self.consume(Token::Semicolon, "Expected ';' after value.".to_string());
            self.add_code(OpCode::Defer);
        // FIXME: defer/debug
        } else if self.check_current(Token::LeftCurlyBrace) {
            self.begin_scope();
            self.block();
            self.end_scope();
        } else {
            self.expr_stmt();
        }
    }

    fn begin_scope(&mut self) {
        self.scope.depth += 1;
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

    fn is_global_scope(&self) -> bool {
        self.scope.depth == 0
    }

    fn block(&mut self) {
        while !self.check(Token::RightCurlyBrace) && !self.check(Token::Eof) {
            self.decl();
        }

        self.consume(Token::RightCurlyBrace, "Expected } after block".to_string());
    }

    fn expr_stmt(&mut self) {
        self.expr();
        self.consume(Token::Semicolon, "Expected semicolon".to_string());
        self.add_code(OpCode::Pop);
    }

    fn expr(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn parse_var(&mut self, msg: String) -> &str {
        self.consume(Token::Identifier, msg);
        &self.prev().literal
    }

    fn decl_scoped_var(&mut self, name: String) {
        let mut err = None;
        for var in &self.scope.vars {
            if var.depth != -1 && var.depth < self.scope.depth as isize {
                break;
            } else if name == var.name {
                err = Some("Already a variable with this name in this scope.".to_string());
            }
        }

        if let Some(err) = err {
            self.err(err);
        }

        if self.is_global_scope() {
            return;
        }

        self.scope.add_var(name);
    }

    fn parse_type(&mut self) -> Option<ValType> {
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
            _ => return None,
        };

        self.advance();

        Some(val_type)
    }

    fn string(&mut self, _: bool) {
        let string = Value::String(self.prev().literal.clone());
        self.add_code(OpCode::String(string))
    }

    fn int(&mut self, _: bool) {
        let int = Value::Int(self.prev().literal.parse::<isize>().unwrap());
        self.add_code(OpCode::IntLiteral(int))
    }

    fn float(&mut self, _: bool) {
        let float = Value::Float64(self.prev().literal.parse::<f64>().unwrap());
        self.add_code(OpCode::FloatLiteral(float))
    }

    fn var(&mut self, assign: bool) {
        self.named_var(assign);
    }

    // FIXME: refactor the logic
    fn named_var(&mut self, assign: bool) {
        let name = self.prev().literal.clone();
        let index = self.scope.resolve(&name);

        if assign && self.check_current(Token::Equal) {
            self.expr();

            let code = if let Some(i) = index {
                OpCode::SetLocal(i)
            } else {
                OpCode::SetGlobal(name)
            };

            self.add_code(code)
        } else {
            let code = if let Some(i) = index {
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
        self.consume(Token::RightParen, str::to_string("Expected parens.")); //FIXME
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

        self.add_code(code)
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

    fn binary(&mut self, _assign: bool) {
        let operator = self.prev().token;
        let precedence = self.rule(&operator).2;

        self.parse_precedence(precedence.next());

        let code = match operator {
            Token::Plus => OpCode::Add,
            Token::Minus => OpCode::Subtract,
            Token::Asterisk => OpCode::Multiply,
            Token::Slash => OpCode::Divide,
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

        self.add_code(code)
    }

    fn literal(&mut self, _: bool) {
        let code = match self.prev().token {
            Token::True => OpCode::Bool(Value::Bool(true)),
            Token::False => OpCode::Bool(Value::Bool(false)),
            _ => {
                return;
            }
        };

        self.add_code(code)
    }

    fn add_code(&mut self, code: OpCode) {
        let pos = self.prev().pos;
        self.chunk.write(code, pos);
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

#[derive(Debug)]
struct Scope {
    vars: Vec<Local>,
    depth: usize,
}

impl Scope {
    fn new() -> Self {
        Self {
            vars: Vec::new(),
            depth: 0,
        }
    }

    fn add_var(&mut self, name: String) {
        self.vars.push(Local { name, depth: -1 });
    }

    fn resolve(&self, name: &str) -> Option<usize> {
        for i in (0..self.vars.len()).rev() {
            if self.vars[i].name == *name {
                return Some(i);
            }
        }

        None
    }

    fn init_last(&mut self) {
        let last = self.vars.pop();
        if let Some(mut last) = last {
            last.depth = self.depth as isize;
            self.vars.push(last);
        }
    }
}

#[derive(Debug)]
struct Local {
    name: String,
    depth: isize,
}
