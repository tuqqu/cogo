use std::fmt;

pub(crate) use self::token::Token;

mod token;

type Literal = String;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Lexeme {
    pub(crate) token: Token,
    pub(crate) pos: Pos,
    pub(crate) literal: Literal,
}

impl Lexeme {
    pub(super) fn new(token: Token, pos: Pos) -> Self {
        Self::new_with_literal(token, pos, Literal::from(""))
    }

    pub(super) fn new_with_literal(token: Token, pos: Pos, literal: Literal) -> Self {
        Self {
            token,
            pos,
            literal,
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub(crate) struct Pos(pub usize, pub usize);

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
