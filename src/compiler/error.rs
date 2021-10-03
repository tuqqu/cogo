use std::{error, fmt};

use super::ValType;
use crate::lex::lexeme::Pos;

/// Errors occurred during the compilation process
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CompileError(pub String, pub Pos);

impl error::Error for CompileError {}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compile error: {} at {}", self.0, self.1)
    }
}

/// Trait for defining custom error handling
pub trait ErrorHandler {
    fn on_error(&mut self, errs: &[Box<dyn error::Error>]);
}

/// Standard handler that outputs errors to stderr and terminates the process
pub struct ToStderrErrorHandler;

impl ErrorHandler for ToStderrErrorHandler {
    fn on_error(&mut self, errs: &[Box<dyn error::Error>]) {
        for err in errs {
            eprintln!("\x1b[0;31m{}\x1b[0m", err);
        }

        std::process::exit(1);
    }
}

/// Errors occurred during incompatible value operations
pub struct TypeError(pub String);

impl TypeError {
    pub(crate) fn wrong_operand_type<T>(expected: &T, actual: &ValType) -> Self
    where
        T: fmt::Display + ?Sized,
    {
        Self(format!(
            "Operand must be of type \"{}\", but got \"{}\"",
            expected, actual,
        ))
    }

    pub(crate) fn expected_same_type_operands<T>(lhs: &T, rhs: &ValType) -> Self
    where
        T: fmt::Display + ?Sized,
    {
        Self(format!(
            "Both operands must be of same type, got \"{}\" and \"{}\"",
            lhs, rhs,
        ))
    }
}

/// Errors occurred during the definition of any named values
pub(crate) struct DefinitionError;
