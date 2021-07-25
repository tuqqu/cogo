use std::mem::discriminant;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    // Nil,

    Bool(bool),

    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int(isize),

    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint(usize),
    Uintptr(usize),

    Float32(f32),
    Float64(f64),

    Complex64(f32, f32),
    Complex128(f64, f64),

    String(String),
}

pub struct TypeError(pub String); //FIXME: add proper error struct
type OperationResult<T> = Result<T, TypeError>;

impl Value {
    pub fn plus_noop(&self) -> OperationResult<()> {
        use Value::*;
        match self {
            Int8(_)
            | Int16(_)
            | Int32(_)
            | Int64(_)
            | Int(_)
            | Uint8(_)
            | Uint16(_)
            | Uint32(_)
            | Uint64(_)
            | Uintptr(_)
            | Uint(_)
            | Float32(_)
            | Float64(_)
            | Complex64(..)
            | Complex128(..) => {},
            a => {
                return Err(TypeError( //FIXME: add proper error message (types etc)
                    format!(
                        "Operand must be of number type, got \"{}\"",
                        a.to_string(),
                    ),
                ))
            }
        };

        Ok(())
    }

    pub fn negate(&self) -> OperationResult<Self> {
        use Value::*;
        let val = match self {
            Int8(a) => Int8(-*a),
            Int16(a) => Int16(-*a),
            Int32(a) => Int32(-*a),
            Int64(a) => Int64(-*a),
            Int(a) => Int(-*a),
            // Uint8(a) => Uint8(-*a), //FIXME: negate logic for uint
            // Uint16(a) => Uint16(-*a),
            // Uint32(a) => Uint32(-*a),
            // Uint64(a) => Uint64(-*a),
            // Uintptr(a) => Uintptr(-*a),
            // Uint(a) => Uint(-*a),
            Float32(a) => Float32(-*a),
            Float64(a) => Float64(-*a),
            Complex64(a, a_i) => Complex64(-*a, -*a_i),
            Complex128(a, a_i) => Complex128(-*a, -*a_i),
            a => {
                return Err(TypeError( //FIXME: add proper error message (types etc)
                    format!(
                        "Operand must be of number type, got \"{}\"",
                        a.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn add(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Int8(lhs + rhs),
            (Int16(lhs), Int16(rhs)) => Int16(lhs + rhs),
            (Int32(lhs), Int32(rhs)) => Int32(lhs + rhs),
            (Int64(lhs), Int64(rhs)) => Int64(lhs + rhs),
            (Int(lhs), Int(rhs)) => Int(lhs + rhs),
            (Uint8(lhs), Uint8(rhs)) => Uint8(lhs + rhs),
            (Uint16(lhs), Uint16(rhs)) => Uint16(lhs + rhs),
            (Uint32(lhs), Uint32(rhs)) => Uint32(lhs + rhs),
            (Uint64(lhs), Uint64(rhs)) => Uint64(lhs + rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Uintptr(lhs + rhs),
            (Uint(lhs), Uint(rhs)) => Uint(lhs + rhs),
            (Float32(lhs), Float32(rhs)) => Float32(lhs + rhs),
            (Float64(lhs), Float64(rhs)) => Float64(lhs + rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Complex64(lhs + rhs, lhs_i + rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Complex128(lhs + rhs, lhs_i + rhs_i),
            (String(lhs), String(rhs)) => String(lhs.to_string() + rhs),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }
    
    pub fn sub(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Int8(lhs - rhs),
            (Int16(lhs), Int16(rhs)) => Int16(lhs - rhs),
            (Int32(lhs), Int32(rhs)) => Int32(lhs - rhs),
            (Int64(lhs), Int64(rhs)) => Int64(lhs - rhs),
            (Int(lhs), Int(rhs)) => Int(lhs - rhs),
            (Uint8(lhs), Uint8(rhs)) => Uint8(lhs - rhs),
            (Uint16(lhs), Uint16(rhs)) => Uint16(lhs - rhs),
            (Uint32(lhs), Uint32(rhs)) => Uint32(lhs - rhs),
            (Uint64(lhs), Uint64(rhs)) => Uint64(lhs - rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Uintptr(lhs - rhs),
            (Uint(lhs), Uint(rhs)) => Uint(lhs - rhs),
            (Float32(lhs), Float32(rhs)) => Float32(lhs - rhs),
            (Float64(lhs), Float64(rhs)) => Float64(lhs - rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Complex64(lhs - rhs, lhs_i - rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Complex128(lhs - rhs, lhs_i - rhs_i),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn mult(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Int8(lhs * rhs),
            (Int16(lhs), Int16(rhs)) => Int16(lhs * rhs),
            (Int32(lhs), Int32(rhs)) => Int32(lhs * rhs),
            (Int64(lhs), Int64(rhs)) => Int64(lhs * rhs),
            (Int(lhs), Int(rhs)) => Int(lhs * rhs),
            (Uint8(lhs), Uint8(rhs)) => Uint8(lhs * rhs),
            (Uint16(lhs), Uint16(rhs)) => Uint16(lhs * rhs),
            (Uint32(lhs), Uint32(rhs)) => Uint32(lhs * rhs),
            (Uint64(lhs), Uint64(rhs)) => Uint64(lhs * rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Uintptr(lhs * rhs),
            (Uint(lhs), Uint(rhs)) => Uint(lhs * rhs),
            (Float32(lhs), Float32(rhs)) => Float32(lhs * rhs),
            (Float64(lhs), Float64(rhs)) => Float64(lhs * rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Complex64(lhs * rhs, lhs_i * rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Complex128(lhs * rhs, lhs_i * rhs_i),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn div(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Int8(lhs / rhs),
            (Int16(lhs), Int16(rhs)) => Int16(lhs / rhs),
            (Int32(lhs), Int32(rhs)) => Int32(lhs / rhs),
            (Int64(lhs), Int64(rhs)) => Int64(lhs / rhs),
            (Int(lhs), Int(rhs)) => Int(lhs / rhs),
            (Uint8(lhs), Uint8(rhs)) => Uint8(lhs / rhs),
            (Uint16(lhs), Uint16(rhs)) => Uint16(lhs / rhs),
            (Uint32(lhs), Uint32(rhs)) => Uint32(lhs / rhs),
            (Uint64(lhs), Uint64(rhs)) => Uint64(lhs / rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Uintptr(lhs / rhs),
            (Uint(lhs), Uint(rhs)) => Uint(lhs / rhs),
            (Float32(lhs), Float32(rhs)) => Float32(lhs / rhs),
            (Float64(lhs), Float64(rhs)) => Float64(lhs / rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Complex64(lhs / rhs, lhs_i / rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Complex128(lhs / rhs, lhs_i / rhs_i),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn modulo(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Int8(lhs % rhs),
            (Int16(lhs), Int16(rhs)) => Int16(lhs % rhs),
            (Int32(lhs), Int32(rhs)) => Int32(lhs % rhs),
            (Int64(lhs), Int64(rhs)) => Int64(lhs % rhs),
            (Int(lhs), Int(rhs)) => Int(lhs % rhs),
            (Uint8(lhs), Uint8(rhs)) => Uint8(lhs % rhs),
            (Uint16(lhs), Uint16(rhs)) => Uint16(lhs % rhs),
            (Uint32(lhs), Uint32(rhs)) => Uint32(lhs % rhs),
            (Uint64(lhs), Uint64(rhs)) => Uint64(lhs % rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Uintptr(lhs % rhs),
            (Uint(lhs), Uint(rhs)) => Uint(lhs % rhs),
            (Float32(lhs), Float32(rhs)) => Float32(lhs % rhs),
            (Float64(lhs), Float64(rhs)) => Float64(lhs % rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Complex64(lhs % rhs, lhs_i % rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Complex128(lhs % rhs, lhs_i % rhs_i),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn not(&self) -> OperationResult<Self> {
        use Value::*;
        let val = match self {
            Bool(a) => Bool(!*a),
            a => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Operand must be of type bool, got \"{}\"",
                        a.to_string()
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn equal(&self, other: &Self) -> OperationResult<Self> {
        if discriminant(self) != discriminant(other) {
            return Err(TypeError(
                format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    self.to_string(), other.to_string(),
                ),
            ))
        }

        Ok(Value::Bool(self == other))
    }

    pub fn greater(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Bool(lhs > rhs),
            (Int16(lhs), Int16(rhs)) => Bool(lhs > rhs),
            (Int32(lhs), Int32(rhs)) => Bool(lhs > rhs),
            (Int64(lhs), Int64(rhs)) => Bool(lhs > rhs),
            (Int(lhs), Int(rhs)) => Bool(lhs > rhs),
            (Uint8(lhs), Uint8(rhs)) => Bool(lhs > rhs),
            (Uint16(lhs), Uint16(rhs)) => Bool(lhs > rhs),
            (Uint32(lhs), Uint32(rhs)) => Bool(lhs > rhs),
            (Uint64(lhs), Uint64(rhs)) => Bool(lhs > rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Bool(lhs > rhs),
            (Uint(lhs), Uint(rhs)) => Bool(lhs > rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs > rhs),
            (Float64(lhs), Float64(rhs)) => Bool(lhs > rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Bool(lhs > rhs && lhs_i > rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Bool(lhs > rhs && lhs_i > rhs_i),
            (String(lhs), String(rhs)) => Bool(lhs > rhs),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn greater_equal(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Bool(lhs >= rhs),
            (Int16(lhs), Int16(rhs)) => Bool(lhs >= rhs),
            (Int32(lhs), Int32(rhs)) => Bool(lhs >= rhs),
            (Int64(lhs), Int64(rhs)) => Bool(lhs >= rhs),
            (Int(lhs), Int(rhs)) => Bool(lhs >= rhs),
            (Uint8(lhs), Uint8(rhs)) => Bool(lhs >= rhs),
            (Uint16(lhs), Uint16(rhs)) => Bool(lhs >= rhs),
            (Uint32(lhs), Uint32(rhs)) => Bool(lhs >= rhs),
            (Uint64(lhs), Uint64(rhs)) => Bool(lhs >= rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Bool(lhs >= rhs),
            (Uint(lhs), Uint(rhs)) => Bool(lhs >= rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs >= rhs),
            (Float64(lhs), Float64(rhs)) => Bool(lhs >= rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Bool(lhs >= rhs && lhs_i >= rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Bool(lhs >= rhs && lhs_i >= rhs_i),
            (String(lhs), String(rhs)) => Bool(lhs >= rhs),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn less(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Bool(lhs < rhs),
            (Int16(lhs), Int16(rhs)) => Bool(lhs < rhs),
            (Int32(lhs), Int32(rhs)) => Bool(lhs < rhs),
            (Int64(lhs), Int64(rhs)) => Bool(lhs < rhs),
            (Int(lhs), Int(rhs)) => Bool(lhs < rhs),
            (Uint8(lhs), Uint8(rhs)) => Bool(lhs < rhs),
            (Uint16(lhs), Uint16(rhs)) => Bool(lhs < rhs),
            (Uint32(lhs), Uint32(rhs)) => Bool(lhs < rhs),
            (Uint64(lhs), Uint64(rhs)) => Bool(lhs < rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Bool(lhs < rhs),
            (Uint(lhs), Uint(rhs)) => Bool(lhs < rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs < rhs),
            (Float64(lhs), Float64(rhs)) => Bool(lhs < rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Bool(lhs < rhs && lhs_i < rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Bool(lhs < rhs && lhs_i < rhs_i),
            (String(lhs), String(rhs)) => Bool(lhs < rhs),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    pub fn less_equal(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (Int8(lhs), Int8(rhs)) => Bool(lhs <= rhs),
            (Int16(lhs), Int16(rhs)) => Bool(lhs <= rhs),
            (Int32(lhs), Int32(rhs)) => Bool(lhs <= rhs),
            (Int64(lhs), Int64(rhs)) => Bool(lhs <= rhs),
            (Int(lhs), Int(rhs)) => Bool(lhs <= rhs),
            (Uint8(lhs), Uint8(rhs)) => Bool(lhs <= rhs),
            (Uint16(lhs), Uint16(rhs)) => Bool(lhs <= rhs),
            (Uint32(lhs), Uint32(rhs)) => Bool(lhs <= rhs),
            (Uint64(lhs), Uint64(rhs)) => Bool(lhs <= rhs),
            (Uintptr(lhs), Uintptr(rhs)) => Bool(lhs <= rhs),
            (Uint(lhs), Uint(rhs)) => Bool(lhs <= rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs <= rhs),
            (Float64(lhs), Float64(rhs)) => Bool(lhs <= rhs),
            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => Bool(lhs <= rhs && lhs_i <= rhs_i),
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => Bool(lhs <= rhs && lhs_i <= rhs_i),
            (String(lhs), String(rhs)) => Bool(lhs <= rhs),
            (lhs, rhs) => { //FIXME: add proper error message (types etc)
                return Err(TypeError(
                    format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        lhs.to_string(), rhs.to_string(),
                    ),
                ))
            }
        };

        Ok(val)
    }

    fn to_string(&self) -> &'static str {
        use Value::*;
        match self {
            Self::Bool(_) => Self::TYPE_BOOL,
            Self::Int8(_) => Self::TYPE_INT8,
            Self::Int16(_) => Self::TYPE_INT16,
            Self::Int32(_) => Self::TYPE_INT32,
            Self::Int64(_) => Self::TYPE_INT64,
            Self::Int(_) => Self::TYPE_INT,
            Self::Uint8(_) => Self::TYPE_UINT8,
            Self::Uint16(_) => Self::TYPE_UINT16,
            Self::Uint32(_) => Self::TYPE_UINT32,
            Self::Uint64(_) => Self::TYPE_UINT64,
            Self::Uint(_) => Self::TYPE_UINT,
            Self::Uintptr(_) => Self::TYPE_UINTPTR,
            Self::Float32(_) => Self::TYPE_FLOAT32,
            Self::Float64(_) => Self::TYPE_FLOAT64,
            Self::Complex64(..) => Self::TYPE_COMPLEX64,
            Self::Complex128(..) => Self::TYPE_COMPLEX128,
            Self::String(_) => Self::TYPE_STRING,
        }
    }

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
}

