use std::cell::RefCell;
use std::fmt::{Display, Formatter};
use std::mem;
use std::rc::Rc;

use super::ValType;

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

    Array(Array, usize, ValType),

    // Service values
    IntLiteral(isize),
    FloatLiteral(f64),
}

type Array = Rc<RefCell<Vec<Value>>>;

pub struct TypeError(pub String); //FIXME: add proper error struct
type OperationResult<T> = Result<T, TypeError>;

impl Value {
    pub fn new_array(vals: Vec<Self>, size: usize, vtype: ValType) -> Self {
        Self::Array(Rc::new(RefCell::new(vals)), size, vtype)
    }

    pub fn default(vtype: &ValType) -> Self {
        match vtype {
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
            ValType::Array(vtype, size) => Self::new_array(
                vec![Self::default(vtype); *size],
                *size,
                ValType::Array(Box::new(*vtype.clone()), *size),
            ),
            _ => panic!("Cannot construct default value for type {}", vtype),
        }
    }

    #[allow(clippy::single_match)]
    pub fn copy_if_soft_reference(&mut self) {
        match self {
            Self::Array(vals, size, vtype) => {
                let vals = vals.as_ref().borrow().clone();
                *self = Self::Array(Rc::new(RefCell::new(vals)), *size, vtype.clone());
            }
            _ => {}
        };
    }

    pub fn to_usize(&self) -> Option<usize> {
        match self {
            Self::IntLiteral(v) if *v >= 0 => Some(*v as usize),
            Self::Int(v) if *v >= 0 => Some(*v as usize),
            Self::Int8(v) if *v >= 0 => Some(*v as usize),
            Self::Int32(v) if *v >= 0 => Some(*v as usize),
            Self::Int64(v) if *v >= 0 => Some(*v as usize),
            Self::Uintptr(v) => Some(*v as usize),
            Self::Uint(v) => Some(*v as usize),
            Self::Uint8(v) => Some(*v as usize),
            Self::Uint32(v) => Some(*v as usize),
            Self::Uint64(v) => Some(*v as usize),
            _ => None,
        }
    }

    pub fn lose_literal_blindly(&mut self) {
        match self {
            Self::IntLiteral(v) => *self = Self::Int(*v),
            Self::FloatLiteral(v) => *self = Self::Float64(*v),
            _ => {}
        }
    }

    pub fn lose_literal(&mut self, vtype: &ValType) {
        match self {
            Self::IntLiteral(v) => {
                *self = match vtype {
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
                    ValType::Float32 => Self::Float32(*v as f32),
                    ValType::Float64 => Self::Float64(*v as f64),
                    _ => panic!("Wrong type cast {} for integer literal", vtype),
                }
            }
            Self::FloatLiteral(v) => {
                *self = match vtype {
                    ValType::Float32 => Self::Float32(*v as f32),
                    ValType::Float64 => Self::Float64(*v),
                    _ => panic!("Wrong type cast {} for float literal", vtype),
                }
            }
            _ => {}
        }
    }

    pub fn cast_to(&self, vtype: ValType) -> Value {
        use Value::*;
        match *self {
            Int8(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String), //FIXME ass string conversion
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Int16(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Int32(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Int64(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Int(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            IntLiteral(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Uint8(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Uint16(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Uint32(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Uint64(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Uintptr(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Uint(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Float32(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0.0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            Float64(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0.0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            FloatLiteral(v) => match vtype {
                ValType::Bool => Value::Bool(v != 0.0),
                ValType::Int8 => Value::Int8(v as i8),
                ValType::Int16 => Value::Int16(v as i16),
                ValType::Int32 => Value::Int32(v as i32),
                ValType::Int64 => Value::Int64(v as i64),
                ValType::Int => Value::Int(v as isize),
                ValType::Uint8 => Value::Uint8(v as u8),
                ValType::Uint16 => Value::Uint16(v as u16),
                ValType::Uint32 => Value::Uint32(v as u32),
                ValType::Uint64 => Value::Uint64(v as u64),
                ValType::Uint => Value::Uint(v as usize),
                ValType::Uintptr => Value::Uintptr(v as usize),
                ValType::Float32 => Value::Float32(v as f32),
                ValType::Float64 => Value::Float64(v as f64),
                // ValType::String => Value::String(v as String),
                _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
            },
            // String(a) => vtype.cast_to(a),
            _ => panic!("Cannot cast type {} to {}", self.get_type(), vtype),
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
            Int8(a) => *a = -*a,
            Int16(a) => *a = -*a,
            Int32(a) => *a = -*a,
            Int64(a) => *a = -*a,
            Int(a) => *a = -*a,
            IntLiteral(a) => *a = -*a,
            // Uint8(a) => Uint8(-*a), //FIXME: negate logic for uint
            // Uint16(a) => Uint16(-*a),
            // Uint32(a) => Uint32(-*a),
            // Uint64(a) => Uint64(-*a),
            // Uintptr(a) => Uintptr(-*a),
            // Uint(a) => Uint(-*a),
            Float32(a) => *a = -*a,
            Float64(a) => *a = -*a,
            FloatLiteral(a) => *a = -*a,
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
            self.lose_literal(&other.get_type());
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
            (FloatLiteral(lhs), IntLiteral(rhs)) => *lhs += *rhs as f64,
            (Float32(lhs), Float32(rhs)) => *lhs += rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs += *rhs as f32,
            (Float32(lhs), IntLiteral(rhs)) => *lhs += *rhs as f32,
            (Float64(lhs), Float64(rhs)) => *lhs += rhs,
            (Float64(lhs), FloatLiteral(rhs)) => *lhs += rhs,
            (Float64(lhs), IntLiteral(rhs)) => *lhs += *rhs as f64,

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
            self.lose_literal(&other.get_type());
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

            (FloatLiteral(lhs), FloatLiteral(rhs)) => *lhs -= rhs,
            (FloatLiteral(lhs), IntLiteral(rhs)) => *lhs -= *rhs as f64,
            (Float32(lhs), Float32(rhs)) => *lhs -= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs -= *rhs as f32,
            (Float32(lhs), IntLiteral(rhs)) => *lhs -= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => *lhs -= rhs,
            (Float64(lhs), FloatLiteral(rhs)) => *lhs -= rhs,
            (Float64(lhs), IntLiteral(rhs)) => *lhs -= *rhs as f64,

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
            self.lose_literal(&other.get_type());
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

            (FloatLiteral(lhs), FloatLiteral(rhs)) => *lhs *= rhs,
            (FloatLiteral(lhs), IntLiteral(rhs)) => *lhs *= *rhs as f64,
            (Float32(lhs), Float32(rhs)) => *lhs *= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs *= *rhs as f32,
            (Float32(lhs), IntLiteral(rhs)) => *lhs *= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => *lhs *= rhs,
            (Float64(lhs), FloatLiteral(rhs)) => *lhs *= rhs,
            (Float64(lhs), IntLiteral(rhs)) => *lhs *= *rhs as f64,

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
            self.lose_literal(&other.get_type());
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
            (FloatLiteral(lhs), IntLiteral(rhs)) => *lhs /= *rhs as f64,
            (Float32(lhs), Float32(rhs)) => *lhs /= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs /= *rhs as f32,
            (Float32(lhs), IntLiteral(rhs)) => *lhs /= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => *lhs /= rhs,
            (Float64(lhs), FloatLiteral(rhs)) => *lhs /= rhs,
            (Float64(lhs), IntLiteral(rhs)) => *lhs /= *rhs as f64,

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
            self.lose_literal(&other.get_type());
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
            (FloatLiteral(lhs), IntLiteral(rhs)) => *lhs %= *rhs as f64,
            (Float32(lhs), Float32(rhs)) => *lhs %= rhs,
            (Float32(lhs), FloatLiteral(rhs)) => *lhs %= *rhs as f32,
            (Float32(lhs), IntLiteral(rhs)) => *lhs %= *rhs as f32,
            (Float64(lhs), Float64(rhs)) => *lhs %= rhs,
            (Float64(lhs), FloatLiteral(rhs)) => *lhs %= rhs,
            (Float64(lhs), IntLiteral(rhs)) => *lhs %= *rhs as f64,

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

            (IntLiteral(lhs), FloatLiteral(rhs)) => Bool(lhs == &(*rhs as isize)),
            (FloatLiteral(lhs), IntLiteral(rhs)) => {
                Bool((lhs - (*rhs as f64)).abs() < f64::EPSILON)
            }

            (Float32(lhs), FloatLiteral(rhs)) => Bool((lhs - (*rhs as f32)).abs() < f32::EPSILON),
            (Float32(lhs), IntLiteral(rhs)) => Bool((lhs - (*rhs as f32)).abs() < f32::EPSILON),
            (IntLiteral(lhs), Float32(rhs)) => Bool(lhs == &(*rhs as isize)),

            (Float64(lhs), FloatLiteral(rhs)) => Bool((lhs - rhs).abs() < f64::EPSILON),
            (Float64(lhs), IntLiteral(rhs)) => Bool((lhs - (*rhs as f64)).abs() < f64::EPSILON),
            (IntLiteral(lhs), Float64(rhs)) => Bool(lhs == &(*rhs as isize)),

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
            Self::Array(.., vtype) => vtype.clone(),
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
            Self::FloatLiteral(_) => matches!(v_type, ValType::Float32 | ValType::Float64),
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
            Self::Array(.., array_type) => array_type == v_type,
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
            Self::Float32(f) => format!("{:e}", f),
            Self::Float64(f) => format!("{:e}", f),
            Self::FloatLiteral(f) => format!("{:e}", f),
            Self::Complex64(c, i) => format!("({:e}+{:e}i)", c, i),
            Self::Complex128(c, i) => format!("({:e}+{:e}i)", c, i),
            Self::String(s) => s.clone(),
            Self::Array(array, _size, vtype) => {
                format!(
                    "<{}>[{}]",
                    vtype,
                    array
                        .as_ref()
                        .borrow()
                        .iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            //FIXME add function tostring (via internal id)
            t => {
                dbg!(t);
                panic!("Unknown type")
            }
        };

        write!(f, "{}", val)
    }
}
