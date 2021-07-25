use std::fmt;

use crate::lexer::lexeme::Pos;
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    // Binary
    Add,
    Subtract,
    Multiply,
    Divide,

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Unary
    Negate,
    PlusNoop,
    Not,

    Constant,
    Return,
    Pop,

    Bool(Value),
    Int(Value),
    Float(Value),
    String(Value),
    Nil,
}

#[derive(Clone)]
pub struct Chunk {
    codes: Vec<OpCode>,
    pos: Vec<Pos>,
}

impl Default for Chunk {
    fn default() -> Self {
        Self::new()
    }
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            codes: vec![],
            pos: vec![],
        }
    }

    pub fn write(&mut self, byte: OpCode, pos: Pos) {
        self.codes.push(byte);
        self.pos.push(pos);
    }

    pub fn codes(&self) -> &[OpCode] {
        &self.codes
    }

    //FIXME: do we need it?

    // pub fn clear(&mut self) {
    //     self.code.clear();
    // }
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for (i, code) in self.codes.iter().enumerate() {
            buffer += &format!("#{:#04b}: {:?} {}\n", i, code, self.pos[i],);
        }

        write!(f, "{}", buffer)
    }
}
