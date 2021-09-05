use std::fmt::Formatter;
use std::{fmt, io};

use super::name_table::NameError;
use super::stack::StackUnderflow;
use crate::compiler::unit::CompilationUnit;
use crate::compiler::{TypeError, ValType};

/// Errors occurred in the virtual machine runtime.
#[derive(Debug)]
pub enum VmError {
    /// Most of the errors belong here as it is the runtime that produces them
    Runtime(String),
    /// Despite the runtime nature of the errors, some of them does not make
    /// much sense to users and signify the compilation errors that were not caught during the compilation.
    Compile(String),
}

impl VmError {
    // -----
    // Type errors

    /// Generic type error, when the actual type does not match the one that was expected
    pub(super) fn type_error(expected: &ValType, actual: &ValType) -> Self {
        Self::Runtime(format!(
            "Got value of type \"{}\" but expected type \"{}\"",
            actual.name(),
            expected.name(),
        ))
    }

    /// Error in function return value type
    pub(super) fn return_type_error(expected: Option<&ValType>, actual: Option<&ValType>) -> Self {
        let msg = match (expected, actual) {
            (Some(expected), Some(actual)) => format!(
                "Function must return \"{}\", got type \"{}\".",
                expected.name(),
                actual.name(),
            ),
            (Some(expected), None) => format!("Function must return \"{}\".", expected.name()),
            (None, _) => "Function must not return value.".to_string(),
        };

        Self::Runtime(msg)
    }

    /// When non-boolean values are being used in conditional statements where only booleans
    /// makes sense. `if cond` or `for cond` are the examples
    pub(super) fn non_bool_in_condition(actual: &ValType) -> Self {
        Self::Runtime(format!(
            "Type \"{}\" used in condition, expected \"bool\"",
            actual.name(),
        ))
    }

    /// Accessing an array or a slice with a non-integer type of index.
    /// Indices may only be any of integer types: `uint`, `int8` etc.
    pub(super) fn index_type_error(actual: &ValType) -> Self {
        Self::Runtime(format!(
            "Indices must be of integer types, got \"{}\"",
            actual.name(),
        ))
    }

    // -----

    pub(super) fn mismatched_argc(expected: usize, actual: u8) -> Self {
        Self::Runtime(format!("Expected {} params, got {}", expected, actual,))
    }

    pub(super) fn undefined(name: &str) -> Self {
        Self::Runtime(format!("Undefined \"{}\".", name))
    }

    pub(super) fn assignment(name: &str) -> Self {
        Self::Runtime(format!("Cannot assign to \"{}\".", name))
    }

    pub(super) fn non_exhaustive_matching_result() -> Self {
        Self::Runtime("No matching value found.".to_string())
    }

    pub(super) fn wrong_array_size(expected: usize, actual: usize) -> Self {
        Self::Runtime(format!(
            "Expected array of size \"{}\", got \"{}\".",
            expected, actual,
        ))
    }

    // ------
    // Compile errors are basically beautified `panic` messages
    // Having them simply means there is an error in the compiler logic

    pub(super) fn iterator_value_expected(actual: &ValType) -> Self {
        Self::Compile(format!("Expected iterator, got type {}", actual.name(),))
    }

    pub(super) fn callable_value_expected(actual: &ValType) -> Self {
        Self::Compile(format!(
            "Trying to call a non-callable value \"{}\"",
            actual.name(),
        ))
    }

    pub(super) fn incorrectly_typed(what: &str, actual: &ValType) -> Self {
        Self::Compile(format!(
            "Value of {} is wrongly typed: \"{}\"",
            what,
            actual.name()
        ))
    }

    pub(super) fn unexpected_matching_result() -> Self {
        Self::Compile("Unexpected matching result".to_string())
    }

    // ------
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

/// Wrong Compilation Unit type encountered.
pub(crate) fn panic_at_cunit_type(cunit: &CompilationUnit) -> ! {
    panic!("Unexpected compilation unit type {}.", cunit.cunit_type())
}
