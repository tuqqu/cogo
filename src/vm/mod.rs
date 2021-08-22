use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::result;

use self::error::VmError;
use self::io::{StdStreamProvider, StreamProvider};
use self::name_table::NameTable;
use self::stack::VmStack;
use crate::compiler::unit::{CompilationUnit as CUnit, FuncUnit};
use crate::compiler::value::Value;
use crate::compiler::OpCode;

mod error;
pub mod io;
mod name_table;
mod stack;

enum VmNamedValue {
    Var(Value),
    Const(Value),
}

type VmRuntimeCall = std::result::Result<(), VmError>;

pub struct Vm {
    globals: HashMap<String, VmNamedValue>,
    names: NameTable<FuncUnit>,
    std_streams: Box<dyn StreamProvider>,
    stack: VmStack<Value>,
    frames: VmStack<Rc<RefCell<CUnitFrame>>>,
    current_frame: usize,
}

impl Vm {
    pub fn new(std_streams: Option<Box<dyn StreamProvider>>, entry_frame: CUnitFrame) -> Self {
        let mut frames = VmStack::new();
        frames.push(Rc::new(RefCell::new(entry_frame)));

        Self {
            globals: HashMap::new(),
            names: NameTable::new(),
            stack: VmStack::new(),
            frames,
            current_frame: 0,
            std_streams: std_streams.unwrap_or_else(|| Box::new(StdStreamProvider::new(None))),
        }
    }

    pub fn current_frame(&self) -> Ref<CUnitFrame> {
        let last_frame = self.frames.retrieve_at(self.current_frame);
        last_frame.borrow()
    }

    pub fn current_frame_mut(&mut self) -> RefMut<CUnitFrame> {
        let last_frame = self.frames.retrieve_at(self.current_frame);
        last_frame.borrow_mut()
    }

    pub fn run(&mut self) -> VmResult {
        let mut match_val: Option<Value> = None;
        let mut switches: VmStack<Switch> = VmStack::new();

        loop {
            let op_code = self.current_frame().next().clone();
            let op_code = if let Some(op_code) = op_code {
                op_code
            } else if self.current_frame == 0 {
                break;
            } else {
                self.current_frame -= 1;
                self.frames.pop()?;
                continue;
            };
            // eprintln!("\x1b[0;35m{:?}\x1b[0m", op_code);

            match op_code {
                OpCode::Noop => {}
                OpCode::PlusNoop => {
                    let a = self.stack.pop()?;
                    a.plus_noop()?;
                    self.stack.push(a);
                }
                OpCode::Negate => {
                    let a = self.stack.pop()?;
                    self.stack.push(a.negate()?);
                }
                OpCode::Subtract => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.sub(&b)?);
                }
                OpCode::Add => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.add(&b)?);
                }
                OpCode::Multiply => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.mult(&b)?);
                }
                OpCode::Divide => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.div(&b)?);
                }
                OpCode::Remainder => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.modulo(&b)?);
                }
                OpCode::Return(void) => {
                    let val = if !void { Some(self.stack.pop()?) } else { None };

                    if !self.return_conforms(&val) {
                        panic!("error in return type");
                    }

                    self.discard_frame_stack()?;
                    self.current_frame -= 1;
                    self.frames.pop()?;

                    if !void {
                        self.stack.push(val.unwrap());
                    }

                    continue;
                }
                OpCode::Exit => {
                    return Ok(());
                }
                OpCode::Bool(v) | OpCode::Int(v) | OpCode::Float(v) | OpCode::String(v) => {
                    self.stack.push(v.clone()); //FIXME clone
                }
                // FIXME literals
                OpCode::IntLiteral(v) | OpCode::FloatLiteral(v) => {
                    self.stack.push(v.clone()); //FIXME clone
                }
                OpCode::Func(funit) => {
                    if let CUnit::Function(func) = funit {
                        self.names.insert(func.name.clone(), func.clone())?;
                        self.stack.push(Value::Func(func.name.clone())); //FIXME clone
                    } else {
                        panic!("func expected");
                    }
                }
                OpCode::Not => {
                    let a = self.stack.pop()?;
                    self.stack.push(a.not()?);
                }
                OpCode::Equal => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.equal(&b)?);
                }
                OpCode::NotEqual => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.equal(&b)?.not()?);
                }
                OpCode::Greater => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.greater(&b)?);
                }
                OpCode::GreaterEqual => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.greater_equal(&b)?);
                }
                OpCode::Less => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.less(&b)?);
                }
                OpCode::LessEqual => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    self.stack.push(a.less_equal(&b)?);
                }
                OpCode::Pop => {
                    self.stack.pop()?;
                }
                // FIXME remove defer/debug
                OpCode::Defer => {
                    let val = self.stack.pop()?;
                    writeln!(self.std_streams.stream_out(), "{:?}", val)?;
                }
                OpCode::VarGlobal(name, val_type) => {
                    let value = self.stack.retrieve(); //FIXME improve message when void function result is used

                    if let Some(val_type) = val_type {
                        if !value.is_of_type(&val_type) {
                            return Err(VmError::Compile(format!(
                                "Got value of type \"{}\" but expected type \"{}\".",
                                value.get_type().name(),
                                val_type.name()
                            ))); //FIXME: err msg
                        }
                    }

                    if self.globals.contains_key(&name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" already declared in this block.",
                            name
                        ))); //FIXME: err msg
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Var(value.clone()));
                    self.stack.pop()?;
                }
                OpCode::ConstGlobal(name, val_type) => {
                    let value = self.stack.retrieve();
                    if let Some(val_type) = val_type {
                        if !value.is_of_type(&val_type) {
                            return Err(VmError::Compile(format!(
                                "Got value of type \"{}\" but expected type \"{}\".",
                                value.get_type().name(),
                                val_type.name()
                            ))); //FIXME: err msg
                        }
                    }

                    if self.globals.contains_key(&name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" already declared in this block.",
                            name
                        ))); //FIXME: err msg
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Const(value.clone()));
                    self.stack.pop()?;
                }
                OpCode::GetGlobal(name) => {
                    if let Some(val) = self.globals.get(&name) {
                        let val = match val {
                            VmNamedValue::Var(val) => val,
                            VmNamedValue::Const(val) => val,
                        };
                        self.stack.push(val.clone());
                    } else {
                        return Err(VmError::Compile(format!("Undefined \"{}\".", name)));
                        //FIXME: err msg
                    }
                }
                OpCode::SetGlobal(name) => {
                    if !self.globals.contains_key(&name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" is not previously defined.",
                            name
                        ))); //FIXME: err msg
                    }

                    let value = self.stack.retrieve();
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
                    let offset = self.current_frame().stack_pos;
                    let value = self.stack.retrieve_at(i + offset).clone();
                    self.stack.push(value);
                }
                OpCode::SetLocal(i) => {
                    let offset = self.current_frame().stack_pos;
                    let old_v = self.stack.retrieve_at(i + offset).clone();
                    let value = self.stack.retrieve().clone();

                    if !old_v.same_type(&value) {
                        return Err(VmError::Compile(format!(
                            "Wrong type \"{}\", expected \"{}\".",
                            value.get_type().name(),
                            old_v.get_type().name()
                        ))); //FIXME: err msg
                    }

                    self.stack.put_at(i + offset, value);
                }
                OpCode::Call(argc) => {
                    let val = self.stack.retrieve_by(argc as usize).clone();
                    self.call(&val, argc)?;

                    self.current_frame_mut().inc_pointer(1);
                    self.current_frame += 1;
                    continue;
                }
                OpCode::ValidateType(val_type) => {
                    let val = self.stack.retrieve();
                    if !val.is_of_type(&val_type) {
                        return Err(VmError::Compile(format!(
                            "Wrong type \"{}\", expected \"{}\".",
                            val.get_type().name(),
                            val_type.name()
                        ))); //FIXME: err msg
                    }
                }
                OpCode::ValidateTypeAt(val_type, at) => {
                    let val = self.stack.retrieve_by(at);
                    if !val.is_of_type(&val_type) {
                        return Err(VmError::Compile(format!(
                            "Wrong type \"{}\", expected \"{}\".",
                            val.get_type().name(),
                            val_type.name()
                        ))); //FIXME: err msg
                    }
                }
                OpCode::PutDefaultValue(val_type) => {
                    self.stack.push(Value::default(&val_type));
                }
                OpCode::IfFalseJump(j) => {
                    let value = self.stack.retrieve();
                    match value {
                        Value::Bool(false) => {
                            self.current_frame_mut().inc_pointer(j);
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
                    self.current_frame_mut().inc_pointer(j);
                }
                OpCode::BackJump(j) => {
                    self.current_frame_mut().dec_pointer(j);
                }
                OpCode::DefaultJump(j) => {
                    let last = switches.last_mut();

                    if last.matched {
                        switches.pop()?;
                    } else {
                        self.current_frame_mut().dec_pointer(j);
                        last.matched = true;
                    }
                }
                OpCode::CaseBreakJump(j) => {
                    let mut last = switches.last_mut();
                    if last.jump_from_case {
                        self.current_frame_mut().inc_pointer(j);
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
                    let val = self.stack.pop()?;
                    match_val = Some(val);
                    switches.push(Switch::new());
                }
                OpCode::DefaultCaseJump(j) => {
                    let mut last = switches.last_mut();
                    if !last.fall_flag {
                        self.current_frame_mut().inc_pointer(j);
                    }
                    if last.fall_flag {
                        last.fall_flag = false;
                    }
                }
                OpCode::CaseJump(j) => {
                    let mut last = switches.last_mut();

                    if !last.fall_flag {
                        if let Some(match_val) = &match_val {
                            let val = self.stack.pop()?;

                            match match_val.equal(&val)? {
                                Value::Bool(true) => {
                                    last.matched = true;
                                }
                                Value::Bool(false) => {
                                    self.current_frame_mut().inc_pointer(j);
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

            // eprintln!("\x1b[0;32m{:?}\x1b[0m", self.stack);
            // eprintln!("\x1b[0;37m{:?}\x1b[0m", self.names);

            self.current_frame_mut().inc_pointer(1);
        }

        VmResult::Ok(())
    }

    fn call(&mut self, val: &Value, argc: u8) -> VmRuntimeCall {
        match val {
            Value::Func(name) => {
                let f = self.names.get(name)?;

                if argc as usize != f.params.len() {
                    return Err(VmError::Runtime(format!(
                        "Expected {} params, got {}",
                        f.params.len(),
                        argc
                    ))); //FIXME display
                }

                let mut frame = CUnitFrame::new(CUnit::Function(f.clone()));
                frame.stack_pos = self.stack.len() - argc as usize;
                self.frames.push(Rc::new(RefCell::new(frame)));

                Ok(())
            }
            _ => Err(VmError::Runtime(format!(
                "Trying to call a non-callable value {:?}",
                val
            ))), //FIXME display
        }
    }

    fn discard_frame_stack(&mut self) -> VmResult {
        if self.stack.len() != 0 {
            while self.stack.len() >= self.current_frame().stack_pos {
                self.stack.pop()?;
            }
        }

        Ok(())
    }

    fn return_conforms(&self, val: &Option<Value>) -> bool {
        match &self.current_frame().cunit {
            CUnit::Function(funit) => match (&funit.ret_type, val) {
                (Some(v_type), Some(val)) => val.is_of_type(v_type),
                (None, None) => true,
                _ => false,
            },
            _ => panic!("Wrong cunit type"),
        }
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

#[derive(Debug)]
pub struct CUnitFrame {
    cunit: CUnit,
    pointer: usize,
    stack_pos: usize,
}

impl CUnitFrame {
    pub fn new(cunit: CUnit) -> Self {
        Self {
            cunit,
            pointer: 0,
            stack_pos: 0,
        }
    }

    pub fn inc_pointer(&mut self, by: usize) {
        self.pointer += by;
    }

    pub fn dec_pointer(&mut self, by: usize) {
        self.pointer -= by;
    }

    pub fn next(&self) -> Option<OpCode> {
        if self.pointer >= self.cunit.chunk().codes().len() {
            None
        } else {
            Some(self.cunit.chunk().codes()[self.pointer].clone())
        }
    }
}
