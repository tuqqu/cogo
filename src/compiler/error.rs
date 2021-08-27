use std::{error, fmt};

use crate::lex::lexeme::Pos;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CompileError(pub String, pub Pos);

impl error::Error for CompileError {}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Compile error: {} at {}", self.0, self.1)
    }
}

pub trait ErrorHandler {
    fn on_error(&mut self, errs: &[Box<dyn error::Error>]) -> !;
}

pub struct ToStderrErrorHandler;

impl ErrorHandler for ToStderrErrorHandler {
    fn on_error(&mut self, errs: &[Box<dyn error::Error>]) -> ! {
        for err in errs {
            eprintln!("\x1b[0;31m{}\x1b[0m", err);
        }

        std::process::exit(1);
    }
}
