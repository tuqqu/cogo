use std::fmt;

use super::unit::CompilationUnit;
use super::{ValType, Value};
use crate::lex::lexeme::Pos;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum OpCode {
    // Control
    Defer,
    Noop,
    Pop,

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

    //Misc
    Call(u8),

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
    Return(bool),
    Exit,

    Bool(Value),
    Int(Value),
    Float(Value),
    String(Value),
    Func(CompilationUnit),
    Nil,

    // FIXME remove Value from here
    IntLiteral(Value),
    FloatLiteral(Value),

    VarGlobal(String, Option<ValType>),
    ConstGlobal(String, Option<ValType>),

    GetGlobal(String),
    SetGlobal(String),
    GetLocal(usize),
    SetLocal(usize),

    ValidateType(ValType),
    ValidateTypeAt(ValType, usize), //FIXME
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
    pub(crate) fn new() -> Self {
        Self {
            codes: vec![],
            pos: vec![],
        }
    }

    pub(crate) fn write(&mut self, op_code: OpCode, pos: Pos) -> usize {
        self.codes.push(op_code);
        self.pos.push(pos);

        self.codes.len() - 1
    }

    pub(crate) fn write_at(&mut self, at: usize, op_code: OpCode) {
        if self.codes.len() > at {
            self.codes[at] = op_code;
        } else {
            panic!("Trying to overwrite a non-existent op code.");
        }
    }

    pub(crate) fn pop(&mut self) -> Option<OpCode> {
        self.pos.pop();
        self.codes.pop()
    }

    pub(crate) fn codes(&self) -> &[OpCode] {
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
