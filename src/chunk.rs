use std::fmt;

use crate::lexer::lexeme::Pos;
use crate::value::{ValType, Value};

#[derive(Debug, Clone)]
pub enum OpCode {
    Defer,
    Noop,

    // Unary
    Negate,
    PlusNoop,
    Not,

    // Binary
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,

    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Switch,
    DefaultJump(usize),
    CaseJump(usize),
    DefaultCaseJump(usize),
    IfFalseJump(usize),
    Jump(usize),
    BackJump(usize),
    CaseBreakJump(usize),
    DoCaseBreakJump,
    Fallthrough,

    Continue,
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
    ConstGlobal(String, Option<ValType>),

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

    pub fn write(&mut self, byte: OpCode, pos: Pos) -> usize {
        self.codes.push(byte);
        self.pos.push(pos);

        self.codes.len() - 1
    }

    pub fn write_at(&mut self, at: usize, byte: OpCode) {
        if self.codes.len() > at {
            self.codes[at] = byte;
        } else {
            panic!("Trying to overwrite a non-existent op code.");
        }
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
