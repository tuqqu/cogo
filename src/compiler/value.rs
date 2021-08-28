#![allow(dead_code)]
use std::fmt::{Display, Formatter};
use std::mem;

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
    Func(String),
    FuncBuiltin(String),

    // Service values
    IntLiteral(isize),
    FloatLiteral(f64),
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
            _ => panic!("unknown valtye"), //FIXME: change to separate errors
        }
    }

    pub fn lose_literal(&mut self, vtype: Option<ValType>) {
        match self {
            Self::IntLiteral(v) => {
                *self = if let Some(vtype) = vtype {
                    match vtype {
                        ValType::Int => Self::Int(*v),
                        ValType::Int8 => Self::Int8(*v as i8),
                        ValType::Int16 => Self::Int16(*v as i16),
                        ValType::Int32 => Self::Int32(*v as i32),
                        ValType::Int64 => Self::Int64(*v as i64),
                        ValType::Uint => Self::Uint(*v as usize),
                        ValType::Uint8 => Self::Uint8(*v as u8),
                        ValType::Uint16 => Self::Uint16(*v as u16),
                        ValType::Uint32 => Self::Uint32(*v as u32),
                        ValType::Uint64 => Self::Uint64(*v as u64),
                        ValType::Uintptr => Self::Uintptr(*v as usize),
                        _ => panic!("Wrong type for integer literal"),
                    }
                } else {
                    Self::Int(*v)
                }
            }
            Self::FloatLiteral(v) => {
                *self = if let Some(vtype) = vtype {
                    match vtype {
                        ValType::Float32 => Self::Float32(*v as f32),
                        ValType::Float64 => Self::Float64(*v),
                        _ => panic!("Wrong type for float literal"),
                    }
                } else {
                    Self::Float64(*v)
                }
            }
            _ => {}
        }
    }

    fn is_literal(&self) -> bool {
        matches!(self, Self::IntLiteral(_) | Self::FloatLiteral(_))
    }

    pub fn plus_noop(&self) -> OperationResult<()> {
        use Value::*;
        match self {
            Int8(_) | Int16(_) | Int32(_) | Int64(_) | Int(_) | Uint8(_) | Uint16(_)
            | Uint32(_) | Uint64(_) | Uintptr(_) | Uint(_) | IntLiteral(_) | Float32(_)
            | Float64(_) | FloatLiteral(_) | Complex64(..) | Complex128(..) => {}
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

    pub fn negate(&mut self) -> OperationResult<()> {
        use Value::*;
        match self {
            Int8(a) => {
                *a = -*a;
            }
            Int16(a) => {
                *a = -*a;
            }
            Int32(a) => {
                *a = -*a;
            }
            Int64(a) => {
                *a = -*a;
            }
            Int(a) => {
                *a = -*a;
            }
            IntLiteral(a) => {
                *a = -*a;
            }
            // Uint8(a) => Uint8(-*a), //FIXME: negate logic for uint
            // Uint16(a) => Uint16(-*a),
            // Uint32(a) => Uint32(-*a),
            // Uint64(a) => Uint64(-*a),
            // Uintptr(a) => Uintptr(-*a),
            // Uint(a) => Uint(-*a),
            Float32(a) => {
                *a = -*a;
            }
            Float64(a) => {
                *a = -*a;
            }
            FloatLiteral(a) => {
                *a = -*a;
            }
            Complex64(a, a_i) => {
                *a = -*a;
                *a_i = -*a_i;
            }
            Complex128(a, a_i) => {
                *a = -*a;
                *a_i = -*a_i;
            }
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

    pub fn add(&mut self, other: &Self) -> OperationResult<()> {
        if self.is_literal() && !other.is_literal() {
            self.lose_literal(Some(other.get_type()));
        }

        use Value::*;
        match (self, other) {
            (IntLiteral(lhs), IntLiteral(rhs)) => *lhs += rhs,
            (Int8(lhs), IntLiteral(rhs)) => *lhs += *rhs as i8,
            (Int16(lhs), IntLiteral(rhs)) => *lhs += *rhs as i16,
            (Int32(lhs), IntLiteral(rhs)) => *lhs += *rhs as i32,
            (Int64(lhs), IntLiteral(rhs)) => *lhs += *rhs as i64,
            (Int(lhs), IntLiteral(rhs)) => *lhs += rhs,
            (Uint8(lhs), IntLiteral(rhs)) => *lhs += *rhs as u8,
            (Uint16(lhs), IntLiteral(rhs)) => *lhs += *rhs as u16,
            (Uint32(lhs), IntLiteral(rhs)) => *lhs += *rhs as u32,
            (Uint64(lhs), IntLiteral(rhs)) => *lhs += *rhs as u64,
            (Uintptr(lhs), IntLiteral(rhs)) => *lhs += *rhs as usize,
            (Uint(lhs), IntLiteral(rhs)) => *lhs += *rhs as usize,

            (Int8(lhs), Int8(rhs)) => *lhs += rhs,
            (Int16(lhs), Int16(rhs)) => *lhs += rhs,
            (Int32(lhs), Int32(rhs)) => *lhs += rhs,
            (Int64(lhs), Int64(rhs)) => *lhs += rhs,
            (Int(lhs), Int(rhs)) => *lhs += rhs,
            (Uint8(lhs), Uint8(rhs)) => *lhs += rhs,
            (Uint16(lhs), Uint16(rhs)) => *lhs += rhs,
            (Uint32(lhs), Uint32(rhs)) => *lhs += rhs,
            (Uint64(lhs), Uint64(rhs)) => *lhs += rhs,
            (Uintptr(lhs), Uintptr(rhs)) => *lhs += rhs,
            (Uint(lhs), Uint(rhs)) => *lhs += rhs,

            (FloatLiteral(lhs), FloatLiteral(rhs)) => *lhs += rhs,
            (Float32(lhs), Float32(rhs)) => *lhs += rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs += *rhs as f32,
            (Float64(lhs), Float64(rhs)) => {
                *lhs += rhs;
            }
            (Float64(lhs), FloatLiteral(rhs)) => {
                *lhs += rhs;
            }

            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => {
                *lhs += *rhs;
                *lhs_i += *rhs_i;
            }
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                *lhs += *rhs;
                *lhs_i += *rhs_i;
            }
            (String(lhs), String(rhs)) => {
                *lhs = format!("{}{}", lhs, rhs);
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

        Ok(())
    }

    pub fn sub(&mut self, other: &Self) -> OperationResult<()> {
        if self.is_literal() && !other.is_literal() {
            self.lose_literal(Some(other.get_type()));
        }

        use Value::*;
        match (self, other) {
            (IntLiteral(lhs), IntLiteral(rhs)) => *lhs -= rhs,
            (Int8(lhs), IntLiteral(rhs)) => *lhs -= *rhs as i8,
            (Int16(lhs), IntLiteral(rhs)) => *lhs -= *rhs as i16,
            (Int32(lhs), IntLiteral(rhs)) => *lhs -= *rhs as i32,
            (Int64(lhs), IntLiteral(rhs)) => *lhs -= *rhs as i64,
            (Int(lhs), IntLiteral(rhs)) => *lhs -= rhs,
            (Uint8(lhs), IntLiteral(rhs)) => *lhs -= *rhs as u8,
            (Uint16(lhs), IntLiteral(rhs)) => *lhs -= *rhs as u16,
            (Uint32(lhs), IntLiteral(rhs)) => *lhs -= *rhs as u32,
            (Uint64(lhs), IntLiteral(rhs)) => *lhs -= *rhs as u64,
            (Uintptr(lhs), IntLiteral(rhs)) => *lhs -= *rhs as usize,
            (Uint(lhs), IntLiteral(rhs)) => *lhs -= *rhs as usize,

            (Int8(lhs), Int8(rhs)) => *lhs -= rhs,
            (Int16(lhs), Int16(rhs)) => *lhs -= rhs,
            (Int32(lhs), Int32(rhs)) => *lhs -= rhs,
            (Int64(lhs), Int64(rhs)) => *lhs -= rhs,
            (Int(lhs), Int(rhs)) => *lhs -= rhs,
            (Uint8(lhs), Uint8(rhs)) => *lhs -= rhs,
            (Uint16(lhs), Uint16(rhs)) => *lhs -= rhs,
            (Uint32(lhs), Uint32(rhs)) => *lhs -= rhs,
            (Uint64(lhs), Uint64(rhs)) => *lhs -= rhs,
            (Uintptr(lhs), Uintptr(rhs)) => *lhs -= rhs,
            (Uint(lhs), Uint(rhs)) => *lhs -= rhs,

            (FloatLiteral(lhs), FloatLiteral(rhs)) => *lhs += rhs,
            (Float32(lhs), Float32(rhs)) => *lhs -= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs -= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => {
                *lhs -= rhs;
            }
            (Float64(lhs), FloatLiteral(rhs)) => {
                *lhs -= rhs;
            }

            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => {
                *lhs -= *rhs;
                *lhs_i -= *rhs_i;
            }
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                *lhs -= *rhs;
                *lhs_i -= *rhs_i;
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

        Ok(())
    }

    pub fn mult(&mut self, other: &Self) -> OperationResult<()> {
        if self.is_literal() && !other.is_literal() {
            self.lose_literal(Some(other.get_type()));
        }

        use Value::*;
        match (self, other) {
            (IntLiteral(lhs), IntLiteral(rhs)) => *lhs *= rhs,
            (Int8(lhs), IntLiteral(rhs)) => *lhs *= *rhs as i8,
            (Int16(lhs), IntLiteral(rhs)) => *lhs *= *rhs as i16,
            (Int32(lhs), IntLiteral(rhs)) => *lhs *= *rhs as i32,
            (Int64(lhs), IntLiteral(rhs)) => *lhs *= *rhs as i64,
            (Int(lhs), IntLiteral(rhs)) => *lhs *= rhs,
            (Uint8(lhs), IntLiteral(rhs)) => *lhs *= *rhs as u8,
            (Uint16(lhs), IntLiteral(rhs)) => *lhs *= *rhs as u16,
            (Uint32(lhs), IntLiteral(rhs)) => *lhs *= *rhs as u32,
            (Uint64(lhs), IntLiteral(rhs)) => *lhs *= *rhs as u64,
            (Uintptr(lhs), IntLiteral(rhs)) => *lhs *= *rhs as usize,
            (Uint(lhs), IntLiteral(rhs)) => *lhs *= *rhs as usize,

            (Int8(lhs), Int8(rhs)) => *lhs *= rhs,
            (Int16(lhs), Int16(rhs)) => *lhs *= rhs,
            (Int32(lhs), Int32(rhs)) => *lhs *= rhs,
            (Int64(lhs), Int64(rhs)) => *lhs *= rhs,
            (Int(lhs), Int(rhs)) => *lhs *= rhs,
            (Uint8(lhs), Uint8(rhs)) => *lhs *= rhs,
            (Uint16(lhs), Uint16(rhs)) => *lhs *= rhs,
            (Uint32(lhs), Uint32(rhs)) => *lhs *= rhs,
            (Uint64(lhs), Uint64(rhs)) => *lhs *= rhs,
            (Uintptr(lhs), Uintptr(rhs)) => *lhs *= rhs,
            (Uint(lhs), Uint(rhs)) => *lhs *= rhs,

            (FloatLiteral(lhs), FloatLiteral(rhs)) => *lhs += rhs,
            (Float32(lhs), Float32(rhs)) => *lhs *= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs *= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => {
                *lhs *= rhs;
            }
            (Float64(lhs), FloatLiteral(rhs)) => {
                *lhs *= rhs;
            }

            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => {
                *lhs *= *rhs;
                *lhs_i *= *rhs_i;
            }
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                *lhs *= *rhs;
                *lhs_i *= *rhs_i;
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

        Ok(())
    }

    pub fn div(&mut self, other: &Self) -> OperationResult<()> {
        if self.is_literal() && !other.is_literal() {
            self.lose_literal(Some(other.get_type()));
        }

        use Value::*;
        match (self, other) {
            (IntLiteral(lhs), IntLiteral(rhs)) => *lhs /= rhs,
            (Int8(lhs), IntLiteral(rhs)) => *lhs /= *rhs as i8,
            (Int16(lhs), IntLiteral(rhs)) => *lhs /= *rhs as i16,
            (Int32(lhs), IntLiteral(rhs)) => *lhs /= *rhs as i32,
            (Int64(lhs), IntLiteral(rhs)) => *lhs /= *rhs as i64,
            (Int(lhs), IntLiteral(rhs)) => *lhs /= rhs,
            (Uint8(lhs), IntLiteral(rhs)) => *lhs /= *rhs as u8,
            (Uint16(lhs), IntLiteral(rhs)) => *lhs /= *rhs as u16,
            (Uint32(lhs), IntLiteral(rhs)) => *lhs /= *rhs as u32,
            (Uint64(lhs), IntLiteral(rhs)) => *lhs /= *rhs as u64,
            (Uintptr(lhs), IntLiteral(rhs)) => *lhs /= *rhs as usize,
            (Uint(lhs), IntLiteral(rhs)) => *lhs /= *rhs as usize,

            (Int8(lhs), Int8(rhs)) => *lhs /= rhs,
            (Int16(lhs), Int16(rhs)) => *lhs /= rhs,
            (Int32(lhs), Int32(rhs)) => *lhs /= rhs,
            (Int64(lhs), Int64(rhs)) => *lhs /= rhs,
            (Int(lhs), Int(rhs)) => *lhs /= rhs,
            (Uint8(lhs), Uint8(rhs)) => *lhs /= rhs,
            (Uint16(lhs), Uint16(rhs)) => *lhs /= rhs,
            (Uint32(lhs), Uint32(rhs)) => *lhs /= rhs,
            (Uint64(lhs), Uint64(rhs)) => *lhs /= rhs,
            (Uintptr(lhs), Uintptr(rhs)) => *lhs /= rhs,
            (Uint(lhs), Uint(rhs)) => *lhs /= rhs,

            (FloatLiteral(lhs), FloatLiteral(rhs)) => *lhs /= rhs,
            (Float32(lhs), Float32(rhs)) => *lhs /= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs /= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => {
                *lhs /= rhs;
            }
            (Float64(lhs), FloatLiteral(rhs)) => {
                *lhs /= rhs;
            }

            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => {
                *lhs /= *rhs;
                *lhs_i /= *rhs_i;
            }
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                *lhs /= *rhs;
                *lhs_i /= *rhs_i;
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

        Ok(())
    }

    pub fn modulo(&mut self, other: &Self) -> OperationResult<()> {
        if self.is_literal() && !other.is_literal() {
            self.lose_literal(Some(other.get_type()));
        }

        use Value::*;
        match (self, other) {
            (IntLiteral(lhs), IntLiteral(rhs)) => *lhs %= rhs,
            (Int8(lhs), IntLiteral(rhs)) => *lhs %= *rhs as i8,
            (Int16(lhs), IntLiteral(rhs)) => *lhs %= *rhs as i16,
            (Int32(lhs), IntLiteral(rhs)) => *lhs %= *rhs as i32,
            (Int64(lhs), IntLiteral(rhs)) => *lhs %= *rhs as i64,
            (Int(lhs), IntLiteral(rhs)) => *lhs %= rhs,
            (Uint8(lhs), IntLiteral(rhs)) => *lhs %= *rhs as u8,
            (Uint16(lhs), IntLiteral(rhs)) => *lhs %= *rhs as u16,
            (Uint32(lhs), IntLiteral(rhs)) => *lhs %= *rhs as u32,
            (Uint64(lhs), IntLiteral(rhs)) => *lhs %= *rhs as u64,
            (Uintptr(lhs), IntLiteral(rhs)) => *lhs %= *rhs as usize,
            (Uint(lhs), IntLiteral(rhs)) => *lhs %= *rhs as usize,

            (Int8(lhs), Int8(rhs)) => *lhs %= rhs,
            (Int16(lhs), Int16(rhs)) => *lhs %= rhs,
            (Int32(lhs), Int32(rhs)) => *lhs %= rhs,
            (Int64(lhs), Int64(rhs)) => *lhs %= rhs,
            (Int(lhs), Int(rhs)) => *lhs %= rhs,
            (Uint8(lhs), Uint8(rhs)) => *lhs %= rhs,
            (Uint16(lhs), Uint16(rhs)) => *lhs %= rhs,
            (Uint32(lhs), Uint32(rhs)) => *lhs %= rhs,
            (Uint64(lhs), Uint64(rhs)) => *lhs %= rhs,
            (Uintptr(lhs), Uintptr(rhs)) => *lhs %= rhs,
            (Uint(lhs), Uint(rhs)) => *lhs %= rhs,

            (FloatLiteral(lhs), FloatLiteral(rhs)) => *lhs %= rhs,
            (Float32(lhs), Float32(rhs)) => *lhs %= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs %= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => {
                *lhs %= rhs;
            }
            (Float64(lhs), FloatLiteral(rhs)) => {
                *lhs %= rhs;
            }

            (Complex64(lhs, lhs_i), Complex64(rhs, rhs_i)) => {
                *lhs %= *rhs;
                *lhs_i %= *rhs_i;
            }
            (Complex128(lhs, lhs_i), Complex128(rhs, rhs_i)) => {
                *lhs %= *rhs;
                *lhs_i %= *rhs_i;
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

        Ok(())
    }

    pub fn not(&mut self) -> OperationResult<()> {
        use Value::*;
        match self {
            Bool(a) => {
                *a = !*a;
            }
            a => {
                //FIXME: add proper error message (types etc)
                return Err(TypeError(format!(
                    "Operand must be of type bool, got \"{}\"",
                    a.get_type().name()
                )));
            }
        };

        Ok(())
    }

    pub fn equal(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let res = match (self, other) {
            (IntLiteral(lhs), Int8(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Int16(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Int32(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Int64(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Int(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Uint8(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Uint16(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Uint32(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Uint64(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Uintptr(rhs)) => Bool(lhs == &(*rhs as isize)),
            (IntLiteral(lhs), Uint(rhs)) => Bool(lhs == &(*rhs as isize)),

            (IntLiteral(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as isize)),

            (Int8(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as i8)),
            (Int16(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as i16)),
            (Int32(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as i32)),
            (Int64(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as i64)),
            (Int(lhs), IntLiteral(rhs)) => Bool(lhs == rhs),
            (Uint8(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as u8)),
            (Uint16(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as u16)),
            (Uint32(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as u32)),
            (Uint64(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as u64)),
            (Uintptr(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as usize)),
            (Uint(lhs), IntLiteral(rhs)) => Bool(lhs == &(*rhs as usize)),
            _ => {
                if mem::discriminant(self) != mem::discriminant(other) {
                    return Err(TypeError(format!(
                        "Both operands must be of same types, got \"{}\" and \"{}\"",
                        self.get_type().name(),
                        other.get_type().name(),
                    )));
                }

                Value::Bool(self == other)
            }
        };

        Ok(res)
    }

    pub fn greater(&self, other: &Self) -> OperationResult<Self> {
        use Value::*;
        let val = match (self, other) {
            (IntLiteral(lhs), Int8(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Int16(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Int32(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Int64(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Int(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Uint8(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Uint16(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Uint32(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Uint64(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Uintptr(rhs)) => Bool(lhs > &(*rhs as isize)),
            (IntLiteral(lhs), Uint(rhs)) => Bool(lhs > &(*rhs as isize)),

            (IntLiteral(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as isize)),

            (Int8(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as i8)),
            (Int16(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as i16)),
            (Int32(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as i32)),
            (Int64(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as i64)),
            (Int(lhs), IntLiteral(rhs)) => Bool(lhs > rhs),
            (Uint8(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as u8)),
            (Uint16(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as u16)),
            (Uint32(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as u32)),
            (Uint64(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as u64)),
            (Uintptr(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as usize)),
            (Uint(lhs), IntLiteral(rhs)) => Bool(lhs > &(*rhs as usize)),

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

            (FloatLiteral(lhs), FloatLiteral(rhs)) => Bool(lhs > rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs > rhs),
            (Float32(lhs), FloatLiteral(rhs)) => Bool(lhs > &(*rhs as f32)),
            (Float64(lhs), Float64(rhs)) => Bool(lhs > rhs),
            (Float64(lhs), FloatLiteral(rhs)) => Bool(lhs > rhs),

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
            (IntLiteral(lhs), Int8(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Int16(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Int32(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Int64(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Int(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Uint8(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Uint16(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Uint32(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Uint64(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Uintptr(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (IntLiteral(lhs), Uint(rhs)) => Bool(lhs >= &(*rhs as isize)),

            (IntLiteral(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as isize)),
            (Int8(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as i8)),
            (Int16(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as i16)),
            (Int32(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as i32)),
            (Int64(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as i64)),
            (Int(lhs), IntLiteral(rhs)) => Bool(lhs >= rhs),
            (Uint8(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as u8)),
            (Uint16(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as u16)),
            (Uint32(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as u32)),
            (Uint64(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as u64)),
            (Uintptr(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as usize)),
            (Uint(lhs), IntLiteral(rhs)) => Bool(lhs >= &(*rhs as usize)),

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

            (FloatLiteral(lhs), FloatLiteral(rhs)) => Bool(lhs >= rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs >= rhs),
            (Float32(lhs), FloatLiteral(rhs)) => Bool(lhs > &(*rhs as f32)),
            (Float64(lhs), Float64(rhs)) => Bool(lhs >= rhs),
            (Float64(lhs), FloatLiteral(rhs)) => Bool(lhs >= rhs),

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
            (IntLiteral(lhs), Int8(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Int16(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Int32(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Int64(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Int(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Uint8(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Uint16(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Uint32(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Uint64(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Uintptr(rhs)) => Bool(lhs < &(*rhs as isize)),
            (IntLiteral(lhs), Uint(rhs)) => Bool(lhs < &(*rhs as isize)),

            (IntLiteral(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as isize)),
            (Int8(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as i8)),
            (Int16(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as i16)),
            (Int32(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as i32)),
            (Int64(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as i64)),
            (Int(lhs), IntLiteral(rhs)) => Bool(lhs < rhs),
            (Uint8(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as u8)),
            (Uint16(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as u16)),
            (Uint32(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as u32)),
            (Uint64(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as u64)),
            (Uintptr(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as usize)),
            (Uint(lhs), IntLiteral(rhs)) => Bool(lhs < &(*rhs as usize)),

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

            (FloatLiteral(lhs), FloatLiteral(rhs)) => Bool(lhs < rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs < rhs),
            (Float32(lhs), FloatLiteral(rhs)) => Bool(lhs > &(*rhs as f32)),
            (Float64(lhs), Float64(rhs)) => Bool(lhs < rhs),
            (Float64(lhs), FloatLiteral(rhs)) => Bool(lhs < rhs),

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
            (IntLiteral(lhs), Int8(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Int16(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Int32(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Int64(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Int(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Uint8(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Uint16(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Uint32(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Uint64(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Uintptr(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (IntLiteral(lhs), Uint(rhs)) => Bool(lhs <= &(*rhs as isize)),

            (IntLiteral(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as isize)),
            (Int8(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as i8)),
            (Int16(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as i16)),
            (Int32(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as i32)),
            (Int64(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as i64)),
            (Int(lhs), IntLiteral(rhs)) => Bool(lhs <= rhs),
            (Uint8(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as u8)),
            (Uint16(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as u16)),
            (Uint32(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as u32)),
            (Uint64(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as u64)),
            (Uintptr(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as usize)),
            (Uint(lhs), IntLiteral(rhs)) => Bool(lhs <= &(*rhs as usize)),

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

            (FloatLiteral(lhs), FloatLiteral(rhs)) => Bool(lhs <= rhs),
            (Float32(lhs), Float32(rhs)) => Bool(lhs <= rhs),
            (Float32(lhs), FloatLiteral(rhs)) => Bool(lhs > &(*rhs as f32)),
            (Float64(lhs), Float64(rhs)) => Bool(lhs <= rhs),
            (Float64(lhs), FloatLiteral(rhs)) => Bool(lhs <= rhs),

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
            // Self::Func(f) => ValType::Func(), FIXME impl here a transformation of f -> ftype
            Self::IntLiteral(_) => ValType::Int,
            Self::FloatLiteral(_) => ValType::Float64,
            t => {
                dbg!(t);
                panic!("Unknown type")
            }
        }
    }

    pub fn is_of_type(&self, v_type: &ValType) -> bool {
        match &self {
            //FIXME check
            Self::Func(_) => true,
            Self::FloatLiteral(_) => matches!(
                v_type,
                ValType::Float32 | ValType::Float64
            ),
            Self::IntLiteral(_) => matches!(
                v_type,
                ValType::Int
                | ValType::Int8
                | ValType::Int16
                | ValType::Int32
                | ValType::Int64
                | ValType::Uint
                | ValType::Uint8
                | ValType::Uint16
                | ValType::Uint32
                | ValType::Uint64
                | ValType::Uintptr
            ),
            _ => self.get_type() == *v_type,
        }
    }

    pub fn same_type(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Self::Bool(b) => b.to_string(),
            Self::Int8(i) => i.to_string(),
            Self::Int16(i) => i.to_string(),
            Self::Int32(i) => i.to_string(),
            Self::Int64(i) => i.to_string(),
            Self::Int(i) => i.to_string(),
            Self::Uint8(i) => i.to_string(),
            Self::Uint16(i) => i.to_string(),
            Self::Uint32(i) => i.to_string(),
            Self::Uint64(i) => i.to_string(),
            Self::Uint(i) => i.to_string(),
            Self::Uintptr(i) => i.to_string(),
            Self::IntLiteral(i) => i.to_string(),
            Self::Float32(f) => format!("{:.1}", f),
            Self::Float64(f) => format!("{:.1}", f),
            Self::FloatLiteral(f) => format!("{:.1}", f),
            Self::Complex64(c, i) => format!("({:.1}+{:.1}i)", c, i),
            Self::Complex128(c, i) => format!("({:.1}+{:.1}i)", c, i),
            Self::String(s) => s.clone(),
            //FIXME add function tostring (via internal id)
            t => {
                dbg!(t);
                panic!("Unknown type")
            }
        };

        write!(f, "{}", val)
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

impl Display for ValType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
