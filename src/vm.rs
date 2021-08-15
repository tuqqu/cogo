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

    fn retrieve(&self) -> &T {
        self.retrieve_at(self.stack.len() - 1)
    }

    fn retrieve_at(&self, i: usize) -> &T {
        self.stack
            .get(i)
            .expect("Cannot retrieve value from stack.")
    }

    fn last_mut(&mut self) -> &mut T {
        self.stack
            .last_mut()
            .expect("Cannot retrieve mutable reference on an empty stack.")
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

enum VmNamedValue {
    Var(Value),
    Const(Value),
}

pub struct Vm {
    globals: HashMap<String, VmNamedValue>,
    std_streams: Box<dyn StreamProvider>,
}

impl Vm {
    pub fn new(std_streams: Option<Box<dyn StreamProvider>>) -> Self {
        Self {
            globals: HashMap::new(),
            std_streams: std_streams.unwrap_or_else(|| Box::new(StdStreamProvider::new(None))),
        }
    }

    pub fn run(&mut self, chunk: &Chunk) -> VmResult {
        let mut stack = VmStack::<Value>::new();
        let codes = chunk.codes();
        let last = codes.len();
        let mut i = 0;

        let mut match_val: Option<Value> = None;
        let mut switches: VmStack<Switch> = VmStack::new();

        while i < last {
            let op_code = &codes[i];

            match op_code {
                OpCode::Noop => {}
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
                OpCode::Remainder => {
                    let b = stack.pop()?;
                    let a = stack.pop()?;
                    stack.push(a.modulo(&b)?);
                }
                OpCode::Return => {
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
                    let value = stack.retrieve();
                    if let Some(val_type) = val_type {
                        if !value.is_of_type(val_type) {
                            return Err(VmError::Compile(format!(
                                "Got value of type \"{}\" but expected type \"{}\".",
                                value.get_type().name(),
                                val_type.name()
                            ))); //FIXME: err msg
                        }
                    }

                    if self.globals.contains_key(name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" already declared in this block.",
                            name
                        ))); //FIXME: err msg
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Var(value.clone()));
                    stack.pop()?;
                }
                OpCode::ConstGlobal(name, val_type) => {
                    let value = stack.retrieve();
                    if let Some(val_type) = val_type {
                        if !value.is_of_type(val_type) {
                            return Err(VmError::Compile(format!(
                                "Got value of type \"{}\" but expected type \"{}\".",
                                value.get_type().name(),
                                val_type.name()
                            ))); //FIXME: err msg
                        }
                    }

                    if self.globals.contains_key(name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" already declared in this block.",
                            name
                        ))); //FIXME: err msg
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Const(value.clone()));
                    stack.pop()?;
                }
                OpCode::VarGlobalNoInit(name, val_type) => {
                    if self.globals.contains_key(name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" already declared in this block.",
                            name
                        ))); //FIXME: err msg
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Var(Value::default(val_type)));
                    // stack.pop()?;
                }
                OpCode::GetGlobal(name) => {
                    if let Some(val) = self.globals.get(name) {
                        let val = match val {
                            VmNamedValue::Var(val) => val,
                            VmNamedValue::Const(val) => val,
                        };
                        stack.push(val.clone());
                    } else {
                        return Err(VmError::Compile(format!("Undefined \"{}\".", name)));
                        //FIXME: err msg
                    }
                }
                OpCode::SetGlobal(name) => {
                    if !self.globals.contains_key(name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" is not previously defined.",
                            name
                        ))); //FIXME: err msg
                    }

                    let value = stack.retrieve();
                    let old_v = self
                        .globals
                        .insert(name.clone(), VmNamedValue::Var(value.clone()))
                        .unwrap_or_else(|| panic!("Old value of \"{}\" not found.", name));

                    if let VmNamedValue::Const(_) = old_v {
                        return Err(VmError::Compile(format!(
                            "Cannot mutate constant \"{}\".",
                            name
                        ))); //FIXME: err msg
                    }

                    let old_v = match old_v {
                        VmNamedValue::Var(val) => val,
                        VmNamedValue::Const(val) => val,
                    };

                    // FIXME: maybe we should store types in a sep hashtable?
                    if !old_v.same_type(value) {
                        return Err(VmError::Compile(format!(
                            "Wrong type \"{}\", expected \"{}\".",
                            value.get_type().name(),
                            old_v.get_type().name()
                        ))); //FIXME: err msg
                    }
                    // no pop?
                }
                OpCode::GetLocal(i) => {
                    let value = stack.retrieve_at(*i).clone();
                    stack.push(value);
                }
                OpCode::SetLocal(i) => {
                    let value = stack.retrieve().clone();
                    stack.put_at(*i, value);
                }
                OpCode::ValidateType(val_type) => {
                    let val = stack.retrieve();
                    if !val.is_of_type(val_type) {
                        return Err(VmError::Compile(format!(
                            "Wrong type \"{}\", expected \"{}\".",
                            val.get_type().name(),
                            val_type.name()
                        ))); //FIXME: err msg
                    }
                }
                OpCode::PutDefaultValue(val_type) => {
                    stack.push(Value::default(val_type));
                }
                OpCode::IfFalseJump(j) => {
                    let value = stack.retrieve();
                    match value {
                        Value::Bool(false) => {
                            i += j;
                        }
                        Value::Bool(true) => {}
                        val => {
                            return Err(VmError::Compile(format!(
                                "Wrong type \"{}\" in if condition, expected \"bool\".",
                                val.get_type().name()
                            )))
                        } //FIXME: err msg
                    }
                }
                OpCode::Jump(j) => {
                    i += j;
                }
                OpCode::BackJump(j) => {
                    i -= j;
                }
                OpCode::DefaultJump(j) => {
                    let last = switches.last_mut();

                    if last.matched {
                        switches.pop()?;
                    } else {
                        i -= j;
                        last.matched = true;
                    }
                }
                OpCode::CaseBreakJump(j) => {
                    let mut last = switches.last_mut();
                    if last.jump_from_case {
                        i += j;
                        last.jump_from_case = false;
                    }
                }
                OpCode::DoCaseBreakJump => {
                    let mut last = switches.last_mut();
                    last.jump_from_case = true;
                }
                OpCode::Fallthrough => {
                    let mut last = switches.last_mut();
                    last.jump_from_case = false;
                    last.fall_flag = true;
                }
                OpCode::Switch => {
                    let val = stack.pop()?;
                    match_val = Some(val);
                    switches.push(Switch::new());
                }
                OpCode::DefaultCaseJump(j) => {
                    let mut last = switches.last_mut();
                    if !last.fall_flag {
                        i += j;
                    }
                    if last.fall_flag {
                        last.fall_flag = false;
                    }
                }
                OpCode::CaseJump(j) => {
                    let mut last = switches.last_mut();

                    if !last.fall_flag {
                        if let Some(match_val) = &match_val {
                            let val = stack.pop()?;

                            match match_val.equal(&val)? {
                                Value::Bool(true) => {
                                    last.matched = true;
                                }
                                Value::Bool(false) => {
                                    i += j;
                                }
                                _ => panic!("Unexpected matching result."),
                            }
                        } else {
                            panic!("No matching value found.");
                        }
                    } else {
                        last.fall_flag = false;
                    }
                }
                _ => {}
            }
            // eprintln!("\x1b[0;33m{:?}\x1b[0m", op_code);
            // eprintln!("\x1b[0;32m{:?}\x1b[0m", stack);

            i += 1;
        }

        VmResult::Ok(())
    }
}

#[derive(Debug)]
pub enum VmError {
    Compile(String),
    Runtime(String),
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
    fn from(_: Error) -> Self {
        Self::Runtime("Runtime error".to_string())
    }
}

pub type VmResult = result::Result<(), VmError>;

struct Switch {
    matched: bool,
    jump_from_case: bool,
    fall_flag: bool,
}

impl Switch {
    fn new() -> Self {
        Self {
            matched: false,
            jump_from_case: false,
            fall_flag: false,
        }
    }
}
