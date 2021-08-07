use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::io::Error;
use std::rc::Rc;
use std::{io, result};

use crate::chunk::{Chunk, OpCode};
use crate::value::{TypeError, Value};

#[derive(Debug)]
struct VmStack<T> {
    stack: Vec<T>,
}

struct StackUnderflow;
type PopResult<T> = result::Result<T, StackUnderflow>;

impl<T> VmStack<T> {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, v: T) {
        self.stack.push(v);
    }

    fn pop(&mut self) -> PopResult<T> {
        if let Some(v) = self.stack.pop() {
            Ok(v)
        } else {
            Err(StackUnderflow)
        }
    }

    fn peek(&self) -> Option<&T> {
        self.stack.last()
    }

    fn peek_at(&self, i: usize) -> Option<&T> {
        if self.stack.is_empty() {
            None
        } else {
            let last = self.stack.len() - 1;
            if last >= i {
                Some(&self.stack[i])
            } else {
                None
            }
        }
    }

    fn put_at(&mut self, i: usize, v: T) {
        self.stack[i] = v;
    }
}

type WriteStream = dyn io::Write;
type ReadStream = dyn io::Read;

type StdoutStream = Rc<RefCell<WriteStream>>;
type StderrStream = Rc<RefCell<WriteStream>>;
type StdinStream = Rc<RefCell<ReadStream>>;

pub trait StreamProvider {
    fn stream_out(&self) -> RefMut<WriteStream>;
    fn stream_err(&self) -> RefMut<WriteStream>;
    fn stream_in(&self) -> RefMut<ReadStream>;
}

pub struct StdStreamProvider {
    stdout: StdoutStream,
    stderr: StderrStream,
    stdin: StdinStream,
}

pub type StdStreams = (
    Option<StdoutStream>,
    Option<StderrStream>,
    Option<StdinStream>,
);

impl StdStreamProvider {
    pub fn new(streams: Option<StdStreams>) -> Self {
        let streams = streams.unwrap_or((None, None, None));
        let (stdout, stderr, stdin) = (
            streams
                .0
                .unwrap_or_else(|| Rc::new(RefCell::new(std::io::stdout()))),
            streams
                .1
                .unwrap_or_else(|| Rc::new(RefCell::new(std::io::stderr()))),
            streams
                .2
                .unwrap_or_else(|| Rc::new(RefCell::new(std::io::stdin()))),
        );

        Self {
            stdout,
            stderr,
            stdin,
        }
    }
}

impl StreamProvider for StdStreamProvider {
    fn stream_out(&self) -> RefMut<WriteStream> {
        self.stdout.borrow_mut()
    }

    fn stream_err(&self) -> RefMut<WriteStream> {
        self.stderr.borrow_mut()
    }

    fn stream_in(&self) -> RefMut<ReadStream> {
        self.stdin.borrow_mut()
    }
}

pub struct Vm {
    globals: HashMap<String, Value>,
    std_streams: Box<dyn StreamProvider>,
}

// type GlobValue = (String, Value);

// //FIXME check usage
// impl Default for Vm {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Vm {
    pub fn new(std_streams: Option<Box<dyn StreamProvider>>) -> Self {
        Self {
            globals: HashMap::new(),
            std_streams: std_streams.unwrap_or_else(|| Box::new(StdStreamProvider::new(None))),
        }
    }

    pub fn run(&mut self, chunk: &Chunk) -> VmResult {
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
                    // let val = stack.pop()?;
                    // FIXME
                    // dbg!(val);
                    return Ok(());
                }
                OpCode::Bool(v) | OpCode::Int(v) | OpCode::Float(v) | OpCode::String(v) => {
                    stack.push(v.clone()); //FIXME clone
                }
                // FIXME literals
                OpCode::IntLiteral(v) | OpCode::FloatLiteral(v) => {
                    stack.push(v.clone()); //FIXME clone
                }
                OpCode::Not => {
                    let a = stack.pop()?;
                    stack.push(a.not()?);
                }
                OpCode::Equal => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.equal(&b)?);
                }
                OpCode::NotEqual => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.equal(&b)?.not()?);
                }
                OpCode::Greater => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.greater(&b)?);
                }
                OpCode::GreaterEqual => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.greater_equal(&b)?);
                }
                OpCode::Less => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.less(&b)?);
                }
                OpCode::LessEqual => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.less_equal(&b)?);
                }
                OpCode::Pop => {
                    stack.pop()?;
                }
                // FIXME remove defer/debug
                OpCode::Defer => {
                    let val = stack.pop()?;
                    writeln!(self.std_streams.stream_out(), "{:?}", val)?;
                }
                OpCode::VarGlobal(name, val_type) => {
                    let value = stack.peek().expect("Cannot retrieve value from stack.");

                    if let Some(val_type) = val_type {
                        if !value.is_of_type(val_type) {
                            eprintln!("\x1b[0;31mValue {:?} is of type {:?} but expected type {:?}\x1b[0m", value, value.get_type(), val_type);
                            return Err(VmError::Compile); //FIXME: err msg
                        }
                    }

                    if self.globals.contains_key(name) {
                        eprintln!(
                            "\x1b[0;31m Name \"{}\" already declared in this block.\x1b[0m",
                            name
                        );
                        return Err(VmError::Compile); //FIXME: err msg
                    }

                    self.globals.insert(name.clone(), value.clone());
                    stack.pop()?;
                }
                OpCode::VarGlobalNoInit(name, val_type) => {
                    if self.globals.contains_key(name) {
                        eprintln!(
                            "\x1b[0;31m Name \"{}\" already declared in this block :).\x1b[0m",
                            name
                        );
                        return Err(VmError::Compile); //FIXME: err msg
                    }

                    self.globals.insert(name.clone(), Value::default(val_type));
                    // stack.pop()?;
                }
                OpCode::GetGlobal(name) => {
                    if let Some(val) = self.globals.get(name) {
                        stack.push(val.clone());
                    } else {
                        eprintln!("\x1b[0;31m Undefined \"{}\".\x1b[0m", name);
                        return Err(VmError::Compile); //FIXME: err msg
                    }
                }
                OpCode::SetGlobal(name) => {
                    if !self.globals.contains_key(name) {
                        eprintln!("\x1b[0;31m{:#?}\x1b[0m", "not prev defined");
                        return Err(VmError::Compile); //FIXME: err msg
                    }

                    let value = stack.peek().expect("Cannot retrieve value from stack");
                    let old_v = self
                        .globals
                        .insert(name.clone(), value.clone())
                        .unwrap_or_else(|| panic!("Old value of \"{}\" not found.", name));

                    // FIXME: maybe we should store types in a sep hashtable?
                    if !old_v.same_type(value) {
                        eprintln!(
                            "\x1b[0;31m Wrong type \"{}\", expected \"{}\".\x1b[0m",
                            value.get_type().name(),
                            old_v.get_type().name()
                        );
                        return Err(VmError::Compile); //FIXME: err msg
                    }
                    // no pop?
                }
                OpCode::GetLocal(i) => {
                    let value = stack
                        .peek_at(*i)
                        .expect("Cannot retrieve value from stack")
                        .clone();
                    stack.push(value);
                }
                OpCode::SetLocal(i) => {
                    let value = stack
                        .peek()
                        .expect("Cannot retrieve value from stack")
                        .clone();
                    stack.put_at(*i, value);
                }
                OpCode::ValidateType(val_type) => {
                    let val = stack.peek().expect("Cannot retrieve value from stack");
                    if !val.is_of_type(val_type) {
                        return Err(VmError::Compile); //FIXME: err msg
                    }
                }
                OpCode::PutDefaultValue(val_type) => {
                    stack.push(Value::default(val_type));
                }
                _ => {}
            }
            // dbg!(op_code);
            // dbg!(&stack);
        }

        Err(VmError::Runtime)
    }
}

#[derive(Debug)]
pub enum VmError {
    Compile,
    Runtime,
}

impl From<StackUnderflow> for VmError {
    fn from(_: StackUnderflow) -> Self {
        eprintln!("\x1b[0;35m{:#?}\x1b[0m", "StackUnderflow");
        Self::Compile
    }
}

impl From<TypeError> for VmError {
    fn from(_: TypeError) -> Self {
        eprintln!("\x1b[0;35m{:#?}\x1b[0m", "Runtime");
        Self::Runtime
    }
}

impl From<io::Error> for VmError {
    fn from(_: Error) -> Self {
        eprintln!("\x1b[0;35m{:#?}\x1b[0m", "std::io::Error");
        Self::Runtime
    }
}

pub type VmResult = result::Result<(), VmError>;
