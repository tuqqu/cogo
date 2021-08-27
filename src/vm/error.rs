use std::fmt::Formatter;
use std::{fmt, io};

use super::name_table::NameError;
use super::stack::StackUnderflow;
use crate::compiler::TypeError;

#[derive(Debug)]
pub enum VmError {
    Compile(String),
    Runtime(String),
}

impl From<StackUnderflow> for VmError {
    fn from(_: StackUnderflow) -> Self {
        Self::Compile("Stack Underflow error".to_string())
    }
}

impl From<TypeError> for VmError {
    fn from(e: TypeError) -> Self {
        Self::Runtime(e.0)
    }
}

impl From<io::Error> for VmError {
    fn from(_: io::Error) -> Self {
        Self::Runtime("Runtime error".to_string())
    }
}

impl From<NameError> for VmError {
    fn from(e: NameError) -> Self {
        Self::Runtime(e.0)
    }
}

impl fmt::Display for VmError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let msg = match &self {
            Self::Compile(s) => s,
            Self::Runtime(s) => s,
        };

        write!(f, "{}", msg)
    }
}
