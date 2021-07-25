use crate::chunk::{Chunk, OpCode};
use crate::lexer::lexeme::{Lexeme, Token};
use crate::value::Value;

pub struct Parser<'a> {
    lexemes: &'a [Lexeme],
    current: usize,
    chunk: Chunk,
    errs: Vec<String>,
    panic: bool,
}

type ParseRule<T> = (Option<fn(&mut T)>, Option<fn(&mut T)>, Precedence);

impl<'a> Parser<'a> {
    pub fn new(lexemes: &'a [Lexeme]) -> Self {
        Self {
            lexemes,
            current: 0,
            chunk: Chunk::new(),
            errs: Vec::new(),
            panic: false,
        }
    }

    fn rule(&self, t: &Token) -> ParseRule<Self> {
        match t {
            Token::LeftParen => (Some(Parser::group), None, Precedence::None),
            Token::RightParen => (None, None, Precedence::None),
            Token::RightCurlyBrace => (None, None, Precedence::None),
            Token::Comma => (None, None, Precedence::None),
            Token::Dot => (None, None, Precedence::None),
            Token::Minus => (Some(Parser::unary), Some(Parser::binary), Precedence::Term),
            Token::Plus => (Some(Parser::unary), Some(Parser::binary), Precedence::Term),
            Token::Semicolon => (None, None, Precedence::None),
            Token::Slash => (None, Some(Parser::binary), Precedence::Factor),
            Token::Asterisk => (None, Some(Parser::binary), Precedence::Factor),
            Token::Bang => (Some(Parser::unary), None, Precedence::None),
            Token::BangEqual => (None, Some(Parser::binary), Precedence::Equality),
            Token::Equal => (None, None, Precedence::None),
            Token::EqualEqual => (None, Some(Parser::binary), Precedence::Comparison),
            Token::Greater => (None, Some(Parser::binary), Precedence::Comparison),
            Token::GreaterEqual => (None, Some(Parser::binary), Precedence::Comparison),
            Token::Less => (None, Some(Parser::binary), Precedence::Comparison),
            Token::LessEqual => (None, Some(Parser::binary), Precedence::Comparison),
            Token::Identifier => (None, None, Precedence::None),
            Token::StringLiteral => (Some(Parser::string), None, Precedence::None),
            Token::IntLiteral => (Some(Parser::int), None, Precedence::None),
            Token::FloatLiteral => (Some(Parser::float), None, Precedence::None),
            Token::LogicAnd => (None, None, Precedence::None),
            Token::Struct => (None, None, Precedence::None),
            Token::Else => (None, None, Precedence::None),
            Token::For => (None, None, Precedence::None),
            Token::Func => (None, None, Precedence::None),
            Token::If => (None, None, Precedence::None),
            // Token::Nil => (Some(Parser::unary), None, Precedence::None),
            Token::False => (Some(Parser::literal), None, Precedence::None),
            Token::True => (Some(Parser::literal), None, Precedence::None),
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

    fn consume(&mut self, token: Token, msg: String) {
        if self.current().token == token {
            self.advance();
        } else {
            self.err(msg);
        }
    }

    fn consume_optionally(&mut self, token: Token) {
        if self.current().token == token {
            self.advance();
        }
    }

    fn current_is(&mut self, tok: Token) -> bool {
        if !self.check(tok) {
            return false;
        }

        self.advance();

        true
    }

    fn check(&mut self, tok: Token) -> bool {
        self.current().token == tok
    }

    fn err(&mut self, msg: String) {
        self.panic = true;
        eprintln!("\x1b[0;31m error {:#?}\x1b[0m", msg); //FIXME: consider passing a handler
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

    fn advance(&mut self) {
        self.current += 1;
    }

    pub fn parse(&mut self) -> Chunk {
        while !self.current_is(Token::Eof) {
            self.decl();
        }

        self.chunk.clone()
    }

    fn decl(&mut self) {
        self.stmt();
        if self.panic {
            self.recover();
        }
    }

    fn stmt(&mut self) {
        if self.current_is(Token::Var) {
            //FIXME: change just to checking here with match wihtout is token and check func
            self.stmt_var();
        } else {
            self.expr_stmt();
        }
    }

    fn expr_stmt(&mut self) {
        self.expr();
        self.consume_optionally(Token::Semicolon);
        self.add_code(OpCode::Pop);
    }

    fn expr(&mut self) {
        self.parse_precedence(Precedence::Assignment)
    }

    fn stmt_var(&mut self) {}

    fn string(&mut self) {
        let string = Value::String(self.lexemes[self.current - 1].literal.clone());
        self.add_code(OpCode::String(string))
    }

    fn int(&mut self) {
        let int = Value::Int(
            self.lexemes[self.current - 1]
                .literal
                .parse::<isize>()
                .unwrap(),
        );
        self.add_code(OpCode::Int(int))
    }

    fn float(&mut self) {
        let float = Value::Float64(
            self.lexemes[self.current - 1]
                .literal
                .parse::<f64>()
                .unwrap(),
        );
        self.add_code(OpCode::Float(float))
    }

    fn group(&mut self) {
        self.expr();
        self.consume(Token::RightParen, str::to_string("expected parens")); //FIXME
    }

    fn unary(&mut self) {
        let operator = &self.lexemes[self.current - 1].token;
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

        prefix.unwrap()(self);

        while prec <= self.rule(&self.current().token).2 {
            self.advance();
            let inflix = self.rule(&self.prev().token).1;
            if inflix.is_none() {
                //FIXME
                return;
            }

            inflix.unwrap()(self);
        }
    }

    fn binary(&mut self) {
        let operator = &self.lexemes[self.current - 1].token;
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

    fn literal(&mut self) {
        let code = match &self.lexemes[self.current - 1].token {
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
