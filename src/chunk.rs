use std::fmt;

use crate::lexer::lexeme::Pos;
use crate::value::{ValType, Value};

#[derive(Debug, Clone)]
pub enum OpCode {
    Defer,
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

    Return,
    Pop,

    Bool(Value),
    Int(Value),
    IntLiteral(Value),
    Float(Value),
    FloatLiteral(Value),
    String(Value),
    Nil,

    VarGlobal(String, Option<ValType>),
    VarGlobalNoInit(String, ValType),
    GetGlobal(String),
    SetGlobal(String),
    GetLocal(usize),
    SetLocal(usize),

    ValidateType(ValType),
    PutDefaultValue(ValType),
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
}

impl fmt::Debug for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();

        for (i, code) in self.codes.iter().enumerate() {
            buffer += &format!("#{}: {:?} {}\n", i, code, self.pos[i],);
        }

        write!(f, "{}", buffer)
    }
}
