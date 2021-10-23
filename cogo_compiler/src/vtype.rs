use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum ValType {
    Nil,
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
    Array(Box<Self>, usize),
    Slice(Box<Self>),
    Func(Box<FuncType>),
    Struct(String),
}

impl ValType {
    // primitives
    const TYPE_NIL: &'static str = "nil";
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
            Self::Nil => str::to_string(Self::TYPE_NIL),
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
            Self::Array(vtype, size) => format!("[{}]{}", size, vtype),
            Self::Slice(vtype) => format!("[]{}", vtype),
            Self::Func(f_type) => f_type.to_string(),
            Self::Struct(name) => str::to_string(name),
        }
    }
}

impl fmt::Display for ValType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeType(Vec<ValType>);

impl CompositeType {
    pub(crate) fn new(types: Vec<ValType>) -> Self {
        Self(types)
    }

    pub(crate) fn new_trivial(vtype: ValType) -> Self {
        Self(vec![vtype])
    }

    pub(crate) fn new_void() -> Self {
        Self(vec![])
    }

    pub(crate) fn is_trivial(&self) -> bool {
        self.0.len() == 1
    }

    pub(crate) fn is_void(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn types(&self) -> &[ValType] {
        &self.0
    }
}

impl fmt::Display for CompositeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_void() {
            write!(f, "")
        } else if self.is_trivial() {
            write!(f, "{}", self.0[0])
        } else {
            write!(
                f,
                "({})",
                self.0
                    .iter()
                    .map(|vt| vt.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )
        }
    }
}

/// Parameter type differs from a value type in that it can be variadic
#[derive(Debug, Clone, PartialEq)]
pub struct ParamType(pub ValType, pub bool);

impl fmt::Display for ParamType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", if self.1 { "..." } else { "" }, self.0)
    }
}

#[derive(Debug, Clone)]
pub struct FuncType {
    args: Vec<ParamType>,
    ret_type: CompositeType,
    variadic: bool,
}

impl FuncType {
    pub fn new(args: Vec<ParamType>, ret_type: CompositeType) -> Self {
        Self {
            variadic: matches!(args.last(), Some(ParamType(_, variadic)) if *variadic),
            args,
            ret_type,
        }
    }

    fn name(&self) -> String {
        format!(
            "{} ({}){}",
            ValType::TYPE_FUNC,
            self.args
                .iter()
                .map(|vt| vt.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.ret_type,
        )
    }

    pub fn args(&self) -> &[ParamType] {
        &self.args
    }

    pub fn ret_type(&self) -> &CompositeType {
        &self.ret_type
    }

    pub fn variadic(&self) -> bool {
        self.variadic
    }
}

impl PartialEq for FuncType {
    fn eq(&self, other: &Self) -> bool {
        self.args == other.args && self.ret_type == other.ret_type
    }
}

impl fmt::Display for FuncType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_type() {
        let ct = CompositeType::new_void();
        assert_eq!("".to_string(), ct.to_string());

        let ct = CompositeType::new_trivial(ValType::Int8);
        assert_eq!("int8".to_string(), ct.to_string());

        let ct = CompositeType::new(vec![
            ValType::Int,
            ValType::String,
            ValType::Array(Box::new(ValType::Float32), 4),
        ]);
        assert_eq!("(int, string, [4]float32)".to_string(), ct.to_string());
    }

    #[test]
    fn test_param_type() {
        let pt = ParamType(ValType::String, true);
        assert_eq!("...string".to_string(), pt.to_string());

        let pt = ParamType(ValType::String, false);
        assert_eq!("string".to_string(), pt.to_string());
    }

    #[test]
    fn test_func_type() {
        let pt_a = ParamType(ValType::String, false);
        let pt_b = ParamType(ValType::Uint, true);

        let ct_a = CompositeType::new_trivial(ValType::Bool);
        let ct_b = CompositeType::new_void();
        let ct_c = CompositeType::new(vec![ValType::Int, ValType::String]);

        let ft = FuncType::new(vec![pt_a.clone(), pt_b.clone()], ct_a);
        assert_eq!("func (string, ...uint)bool".to_string(), ft.to_string());

        let ft = FuncType::new(vec![pt_b.clone()], ct_b);
        assert_eq!("func (...uint)".to_string(), ft.to_string());

        let ft = FuncType::new(vec![pt_a, pt_b], ct_c);
        assert_eq!(
            "func (string, ...uint)(int, string)".to_string(),
            ft.to_string()
        );
    }
}
