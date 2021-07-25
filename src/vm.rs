use crate::chunk::{Chunk, OpCode};
use crate::value::{Value, TypeError};
use std::result;
use std::collections::LinkedList;

#[derive(Debug)]
struct VmStack<T> {
    stack: LinkedList<T>,
}

struct StackUnderflow;
type PopResult<T> = result::Result<T, StackUnderflow>;

impl<T> VmStack<T> {
    fn new() -> Self {
        Self {
            stack: LinkedList::new()
        }
    }

    fn push(&mut self, v: T) {
        self.stack.push_front(v);
    }

    fn pop(&mut self) -> PopResult<T> {
        if let Some(v) = self.stack.pop_front() {
            Ok(v)
        } else {
            Err(StackUnderflow)
        }
    }

    // FIXME: check usage
    fn reset(&mut self) {
        self.stack = LinkedList::new()
    }
}

#[derive(Debug)]
pub struct Vm;

impl Vm {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, chunk: &Chunk) -> VmResult {
        let mut stack = VmStack::<Value>::new();

        for op_code in chunk.codes() {
            match op_code {
                OpCode::PlusNoop => {
                    let a = stack.pop()?;
                    a.plus_noop()?;
                    stack.push(a);
                }
                OpCode::Negate => {
                    let a = stack.pop()?;
                    stack.push(a.negate()?);
                }
                OpCode::Subtract => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.sub(&b)?);
                }
                OpCode::Add => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.add(&b)?);
                }
                OpCode::Multiply => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.mult(&b)?);
                }
                OpCode::Divide => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.div(&b)?);
                }
                OpCode::Return => {
                    let val = stack.pop()?;
                    dbg!(val);
                    return Ok(())
                },
                OpCode::Bool(v)
                | OpCode::Int(v)
                | OpCode::Float(v)
                | OpCode::String(v) => {
                    stack.push(v.clone()); //fixme clone
                }
                OpCode::Not => {
                    let a = stack.pop()?;
                    stack.push(a.not()?);
                },
                OpCode::Equal => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.equal(&b)?);
                },
                OpCode::NotEqual => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.equal(&b)?.not()?);
                },
                OpCode::Greater => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.greater(&b)?);
                },
                OpCode::GreaterEqual => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.greater_equal(&b)?);
                },
                OpCode::Less => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.less(&b)?);
                },
                OpCode::LessEqual => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.less_equal(&b)?);
                },
                OpCode::Pop => {
                    stack.pop()?;
                }
                _ => {},
            }
        }

        Err(VmError::Runtime)
    }
}

pub enum VmError {
    Compile,
    Runtime,
}

impl From<StackUnderflow> for VmError {
    fn from(_: StackUnderflow) -> Self {
        Self::Compile
    }
}

impl From<TypeError> for VmError {
    fn from(_: TypeError) -> Self {
        Self::Runtime
    }
}

pub type VmResult = result::Result<(), VmError>;
