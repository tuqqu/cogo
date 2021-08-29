use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ValType {
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    Int,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint,
    Uintptr,
    Float32,
    Float64,
    Complex64,
    Complex128,
    String,
    Func(Box<FuncType>),
    Struct(String),
}

impl ValType {
    // primitives
    const TYPE_BOOL: &'static str = "bool";
    const TYPE_INT8: &'static str = "int8";
    const TYPE_INT16: &'static str = "int16";
    const TYPE_INT32: &'static str = "int32";
    const TYPE_INT64: &'static str = "int64";
    const TYPE_INT: &'static str = "int";
    const TYPE_UINT8: &'static str = "uint8";
    const TYPE_UINT16: &'static str = "uint16";
    const TYPE_UINT32: &'static str = "uint32";
    const TYPE_UINT64: &'static str = "uint64";
    const TYPE_UINT: &'static str = "uint";
    const TYPE_UINTPTR: &'static str = "uintptr";
    const TYPE_FLOAT32: &'static str = "float32";
    const TYPE_FLOAT64: &'static str = "float64";
    const TYPE_COMPLEX64: &'static str = "complex64";
    const TYPE_COMPLEX128: &'static str = "complex128";
    const TYPE_STRING: &'static str = "string";
    // complex types
    const TYPE_FUNC: &'static str = "func";

    pub fn name(&self) -> String {
        match self {
            Self::Bool => str::to_string(Self::TYPE_BOOL),
            Self::Int8 => str::to_string(Self::TYPE_INT8),
            Self::Int16 => str::to_string(Self::TYPE_INT16),
            Self::Int32 => str::to_string(Self::TYPE_INT32),
            Self::Int64 => str::to_string(Self::TYPE_INT64),
            Self::Int => str::to_string(Self::TYPE_INT),
            Self::Uint8 => str::to_string(Self::TYPE_UINT8),
            Self::Uint16 => str::to_string(Self::TYPE_UINT16),
            Self::Uint32 => str::to_string(Self::TYPE_UINT32),
            Self::Uint64 => str::to_string(Self::TYPE_UINT64),
            Self::Uint => str::to_string(Self::TYPE_UINT),
            Self::Uintptr => str::to_string(Self::TYPE_UINTPTR),
            Self::Float32 => str::to_string(Self::TYPE_FLOAT32),
            Self::Float64 => str::to_string(Self::TYPE_FLOAT64),
            Self::Complex64 => str::to_string(Self::TYPE_COMPLEX64),
            Self::Complex128 => str::to_string(Self::TYPE_COMPLEX128),
            Self::String => str::to_string(Self::TYPE_STRING),
            Self::Func(f_type) => f_type.name(),
            Self::Struct(name) => str::to_string(name),
        }
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone)]
pub struct FuncType {
    args: Vec<ValType>,
    ret_type: Option<ValType>,
}

impl FuncType {
    pub fn new(args: Vec<ValType>, ret_type: Option<ValType>) -> Self {
        Self { args, ret_type }
    }

    fn name(&self) -> String {
        format!(
            "{}({}){}",
            ValType::TYPE_FUNC,
            self.args
                .iter()
                .map(|vt| vt.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            if let Some(ret_type) = &self.ret_type {
                format!(" {}", ret_type.name())
            } else {
                "".to_string()
            }
        )
    }

    #[allow(dead_code)]
    pub fn args(&self) -> &[ValType] {
        &self.args
    }

    pub fn ret_type(&self) -> &Option<ValType> {
        &self.ret_type
    }
}

impl PartialEq for FuncType {
    fn eq(&self, other: &Self) -> bool {
        self.args == other.args && self.ret_type == other.ret_type
    }
}
