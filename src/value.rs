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
    pub fn default(val_type: &ValType) -> Self {
        match val_type {
            ValType::Bool => Self::Bool(false),
            ValType::Int8 => Self::Int8(0),
            ValType::Int16 => Self::Int16(0),
            ValType::Int32 => Self::Int32(0),
            ValType::Int64 => Self::Int64(0),
            ValType::Int => Self::Int(0),
            ValType::Uint8 => Self::Uint8(0),
            ValType::Uint16 => Self::Uint16(0),
            ValType::Uint32 => Self::Uint32(0),
            ValType::Uint64 => Self::Uint64(0),
            ValType::Uint => Self::Uint(0),
            ValType::Uintptr => Self::Uintptr(0),
            ValType::Float32 => Self::Float32(0_f32),
            ValType::Float64 => Self::Float64(0_f64),
            ValType::Complex64 => Self::Complex64(0_f32, 0_f32),
            ValType::Complex128 => Self::Complex128(0_f64, 0_f64),
            ValType::String => Self::String(String::from("")),
            _ => panic!("44"), //FIXME: change to separate errors
        }
    }

    pub fn plus_noop(&self) -> OperationResult<()> {
        use Value::*;
        match self {
            Int8(_) | Int16(_) | Int32(_) | Int64(_) | Int(_) | Uint8(_) | Uint16(_)
            | Uint32(_) | Uint64(_) | Uintptr(_) | Uint(_) | Float32(_) | Float64(_)
            | Complex64(..) | Complex128(..) => {}
            a => {
                return Err(TypeError(
                    //FIXME: add proper error message (types etc)
                    format!(
                        "Operand must be of number type, got \"{}\"",
                        a.get_type().name(),
                    ),
                ));
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
                return Err(TypeError(
                    //FIXME: add proper error message (types etc)
                    format!(
                        "Operand must be of number type, got \"{}\"",
                        a.get_type().name(),
                    ),
                ));
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
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                Complex128(lhs + rhs, lhs_i + rhs_i)
            }
            (String(lhs), String(rhs)) => String(lhs.to_string() + rhs),
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
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
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                Complex128(lhs - rhs, lhs_i - rhs_i)
            }
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
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
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                Complex128(lhs * rhs, lhs_i * rhs_i)
            }
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
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
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                Complex128(lhs / rhs, lhs_i / rhs_i)
            }
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
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
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                Complex128(lhs % rhs, lhs_i % rhs_i)
            }
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
            }
        };

        Ok(val)
    }

    pub fn not(&self) -> OperationResult<Self> {
        use Value::*;
        let val = match self {
            Bool(a) => Bool(!*a),
            a => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Operand must be of type bool, got \"{}\"",
                    a.get_type().name()
                )));
            }
        };

        Ok(val)
    }

    pub fn equal(&self, other: &Self) -> OperationResult<Self> {
        if discriminant(self) != discriminant(other) {
            return Err(TypeError(format!(
                "Both operands must be of same types, got \"{}\" and \"{}\"",
                self.get_type().name(),
                other.get_type().name(),
            )));
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
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
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
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
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
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
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
            (lhs, rhs) => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Both operands must be of same types, got \"{}\" and \"{}\"",
                    lhs.get_type().name(),
                    rhs.get_type().name(),
                )));
            }
        };

        Ok(val)
    }

    pub fn get_type(&self) -> ValType {
        match self {
            Self::Bool(_) => ValType::Bool,
            Self::Int8(_) => ValType::Int8,
            Self::Int16(_) => ValType::Int16,
            Self::Int32(_) => ValType::Int32,
            Self::Int64(_) => ValType::Int64,
            Self::Int(_) => ValType::Int,
            Self::Uint8(_) => ValType::Uint8,
            Self::Uint16(_) => ValType::Uint16,
            Self::Uint32(_) => ValType::Uint32,
            Self::Uint64(_) => ValType::Uint64,
            Self::Uint(_) => ValType::Uint,
            Self::Uintptr(_) => ValType::Uintptr,
            Self::Float32(_) => ValType::Float32,
            Self::Float64(_) => ValType::Float64,
            Self::Complex64(..) => ValType::Complex64,
            Self::Complex128(..) => ValType::Complex128,
            Self::String(_) => ValType::String,
        }
    }

    pub fn is_of_type(&self, v_type: &ValType) -> bool {
        self.get_type() == *v_type
    }

    pub fn same_type(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

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
    Struct(String),
}

impl ValType {
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

    pub fn name(&self) -> &str {
        match self {
            Self::Bool => Self::TYPE_BOOL,
            Self::Int8 => Self::TYPE_INT8,
            Self::Int16 => Self::TYPE_INT16,
            Self::Int32 => Self::TYPE_INT32,
            Self::Int64 => Self::TYPE_INT64,
            Self::Int => Self::TYPE_INT,
            Self::Uint8 => Self::TYPE_UINT8,
            Self::Uint16 => Self::TYPE_UINT16,
            Self::Uint32 => Self::TYPE_UINT32,
            Self::Uint64 => Self::TYPE_UINT64,
            Self::Uint => Self::TYPE_UINT,
            Self::Uintptr => Self::TYPE_UINTPTR,
            Self::Float32 => Self::TYPE_FLOAT32,
            Self::Float64 => Self::TYPE_FLOAT64,
            Self::Complex64 => Self::TYPE_COMPLEX64,
            Self::Complex128 => Self::TYPE_COMPLEX128,
            Self::String => Self::TYPE_STRING,
            Self::Struct(name) => name,
        }
    }
}
