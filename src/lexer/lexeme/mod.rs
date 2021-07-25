use std::fmt;

mod token;
pub use self::token::Token;

type Literal = String;

#[derive(Debug, Clone, PartialEq)]
pub struct Lexeme {
    pub token: Token,
    pub pos: Pos,
    pub literal: Literal
}

impl Lexeme {
    pub fn new(token: Token, pos: Pos) -> Self {
        Self::new_with_literal(token, pos, Literal::from(""))
    }

    pub fn new_with_literal(token: Token, pos: Pos, literal: Literal) -> Self {
        Self {
            token,
            pos,
            literal,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct Pos(pub usize, pub usize);

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.0, self.1)
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self, f)
    }
}

