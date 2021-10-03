pub(crate) mod error;
pub(crate) mod unit;

pub use self::compiler::compile;
pub use self::error::{ErrorHandler, ToStderrErrorHandler, TypeError};
pub use self::opcode::OpCode;
pub use self::unit::{CompilationUnit, FuncUnit};
pub use self::value::Value;
pub use self::vtype::ValType;

pub mod compiler;
mod flow;
mod lex;
mod opcode;
mod scope;
mod structure;
mod value;
mod vtype;
