use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::result;

use self::builtin::FuncBuiltin;
use self::error::VmError;
use self::io::{StdStreamProvider, StreamProvider};
use self::name_table::NameTable;
use self::stack::VmStack;
use crate::compiler::unit::{CompilationUnit as CUnit, FuncUnit};
use crate::compiler::{OpCode, Value};

mod builtin;
mod error;
pub mod io;
mod name_table;
mod stack;

enum VmNamedValue {
    Var(Value),
    Const(Value),
}

impl VmNamedValue {
    fn val(&self) -> &Value {
        match self {
            Self::Var(v) => v,
            Self::Const(v) => v,
        }
    }

    fn val_mut(&mut self) -> &mut Value {
        match self {
            Self::Var(v) => v,
            Self::Const(v) => v,
        }
    }
}

type VmRuntimeCall = std::result::Result<(), VmError>;

pub struct Vm {
    globals: HashMap<String, VmNamedValue>,
    names: NameTable<FuncUnit>,
    builtins: NameTable<FuncBuiltin>,
    std_streams: Box<dyn StreamProvider>,
    stack: VmStack<Value>,
    frames: VmStack<Rc<RefCell<CUnitFrame>>>,
    current_frame: usize,
}

impl Vm {
    pub fn new(std_streams: Option<Box<dyn StreamProvider>>, entry_frame: CUnitFrame) -> Self {
        let mut frames = VmStack::new();
        frames.push(Rc::new(RefCell::new(entry_frame)));

        let mut vm = Self {
            globals: HashMap::new(),
            names: NameTable::new(),
            builtins: NameTable::new(),
            stack: VmStack::new(),
            frames,
            current_frame: 0,
            std_streams: std_streams.unwrap_or_else(|| Box::new(StdStreamProvider::new(None))),
        };

        vm.define_builtins();
        vm
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
                    let mut a = self.stack.pop()?;
                    a.negate()?;
                    self.stack.push(a);
                }
                OpCode::Subtract => {
                    let b = self.stack.pop()?;
                    let mut a = self.stack.pop()?;
                    a.sub(&b)?;
                    self.stack.push(a);
                }
                OpCode::Add => {
                    let b = self.stack.pop()?;
                    let mut a = self.stack.pop()?;
                    a.add(&b)?;
                    self.stack.push(a);
                }
                OpCode::Multiply => {
                    let b = self.stack.pop()?;
                    let mut a = self.stack.pop()?;
                    a.mult(&b)?;
                    self.stack.push(a);
                }
                OpCode::Divide => {
                    let b = self.stack.pop()?;
                    let mut a = self.stack.pop()?;
                    a.div(&b)?;
                    self.stack.push(a);
                }
                OpCode::Remainder => {
                    let b = self.stack.pop()?;
                    let mut a = self.stack.pop()?;
                    a.modulo(&b)?;
                    self.stack.push(a);
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
                OpCode::Bool(v) | OpCode::Int(v) | OpCode::Float(v) | OpCode::String(v) => {
                    self.stack.push(v.clone()); //FIXME clone
                }
                // FIXME literals
                OpCode::IntLiteral(v) | OpCode::FloatLiteral(v) => {
                    self.stack.push(v.clone()); //FIXME clone
                }
                OpCode::Func(funit) => {
                    if let CUnit::Function(func) = funit {
                        self.names.insert(func.name().to_string(), func.clone())?;
                        self.stack.push(Value::Func(func.name().to_string()));
                    } else {
                        panic!("func expected");
                    }
                }
                OpCode::Not => {
                    let mut a = self.stack.pop()?;
                    a.not()?;
                    self.stack.push(a);
                }
                OpCode::Equal => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let eq = a.equal(&b)?;
                    self.stack.push(eq);
                }
                OpCode::NotEqual => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let mut eq = a.equal(&b)?;
                    eq.not()?;
                    self.stack.push(eq);
                }
                OpCode::Greater => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let g = a.greater(&b)?;
                    self.stack.push(g);
                }
                OpCode::GreaterEqual => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let ge = a.greater_equal(&b)?;
                    self.stack.push(ge);
                }
                OpCode::Less => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let l = a.less(&b)?;
                    self.stack.push(l);
                }
                OpCode::LessEqual => {
                    let b = self.stack.pop()?;
                    let a = self.stack.pop()?;
                    let le = a.less_equal(&b)?;
                    self.stack.push(le);
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
                    let mut value = value.clone();
                    if let Some(val_type) = &val_type {
                        if !value.is_of_type(val_type) {
                            return Err(VmError::Compile(format!(
                                "Got value of type \"{}\" but expected type \"{}\".",
                                value.get_type().name(),
                                val_type.name()
                            ))); //FIXME: err msg
                        }
                    }

                    value.lose_literal(val_type);

                    if self.globals.contains_key(&name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" already declared in this block.",
                            name
                        ))); //FIXME: err msg
                    }

                    self.globals.insert(name.clone(), VmNamedValue::Var(value));
                    self.stack.pop()?;
                }
                OpCode::ConstGlobal(name, val_type) => {
                    let value = self.stack.retrieve();
                    let mut value = value.clone();
                    if let Some(val_type) = &val_type {
                        if !value.is_of_type(val_type) {
                            return Err(VmError::Compile(format!(
                                "Got value of type \"{}\" but expected type \"{}\".",
                                value.get_type().name(),
                                val_type.name()
                            ))); //FIXME: err msg
                        }
                    }

                    value.lose_literal(val_type);

                    if self.globals.contains_key(&name) {
                        return Err(VmError::Compile(format!(
                            "Name \"{}\" already declared in this block.",
                            name
                        ))); //FIXME: err msg
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Const(value));
                    self.stack.pop()?;
                }
                OpCode::GetGlobal(name) => {
                    if let Some(nval) = self.globals.get(&name) {
                        self.stack.push(nval.val().clone());
                    } else if let Ok(builtin) = self.builtins.get(&name) {
                        let val = Value::FuncBuiltin(builtin.name().to_string());
                        self.stack.push(val);
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

                    let value = self.stack.retrieve_mut();

                    let old_v = self.globals.get_mut(&name).unwrap();

                    if let VmNamedValue::Const(_) = old_v {
                        return Err(VmError::Compile(format!(
                            "Cannot mutate constant \"{}\".",
                            name
                        ))); //FIXME: err msg
                    }

                    let old_v = old_v.val_mut();
                    // FIXME: maybe we should store types in a sep hashtable?
                    if !old_v.same_type(value) {
                        return Err(VmError::Compile(format!(
                            "Wrong type \"{}\", expected \"{}\".",
                            value.get_type().name(),
                            old_v.get_type().name()
                        ))); //FIXME: err msg
                    }

                    value.lose_literal(Some(old_v.get_type()));
                    *old_v = value.clone();

                    // no pop?
                }
                OpCode::GetLocal(i) => {
                    let offset = self.current_frame().stack_pos;
                    let mut value = self.stack.retrieve_at(i + offset).clone();
                    value.lose_literal(None);
                    self.stack.push(value);
                }
                OpCode::SetLocal(i) => {
                    let offset = self.current_frame().stack_pos;
                    let old_v = self.stack.retrieve_at(i + offset).clone();
                    let mut value = self.stack.retrieve().clone();

                    value.lose_literal(Some(old_v.get_type()));

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
                    match val {
                        Value::Func(name) => {
                            self.call_func(&name, argc)?;
                            self.current_frame_mut().inc_pointer(1);
                            self.current_frame += 1;
                            continue;
                        }
                        Value::FuncBuiltin(name) => {
                            self.call_builtin(&name, argc)?;
                        }
                        _ => {
                            return Err(VmError::Runtime(format!(
                                "Trying to call a non-callable value {:?}",
                                val
                            )))
                        } //FIXME display
                    }
                }
                OpCode::ValidateTypeWithLiteralCast(val_type) => {
                    let val = self.stack.retrieve_mut();
                    val.lose_literal(Some(val_type.clone()));

                    if !val.is_of_type(&val_type) {
                        return Err(VmError::Compile(format!(
                            "Wrong type \"{}\", expected \"{}\".",
                            val.get_type().name(),
                            val_type.name()
                        ))); //FIXME: err msg
                    }
                }
                OpCode::LiteralCast => {
                    let val = self.stack.retrieve_mut();
                    val.lose_literal(None);
                }
                OpCode::ValidateTypeAtWithLiteralCast(val_type, at) => {
                    let val = self.stack.retrieve_by_mut(at);
                    val.lose_literal(Some(val_type.clone()));

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

    fn call_func(&mut self, name: &str, argc: u8) -> VmRuntimeCall {
        let f = self.names.get(name)?;
        if argc as usize != f.param_names().len() {
            return Err(VmError::Runtime(format!(
                "Expected {} params, got {}",
                f.param_names().len(),
                argc
            ))); //FIXME display
        }

        let mut frame = CUnitFrame::new(CUnit::Function(f.clone()));
        frame.stack_pos = self.stack.len() - argc as usize;
        self.frames.push(Rc::new(RefCell::new(frame)));

        Ok(())
    }

    fn call_builtin(&mut self, name: &str, argc: u8) -> VmRuntimeCall {
        let f = self.builtins.get(name)?;
        let len = self.stack.len();
        let stack_pos = len - argc as usize;

        let res = f.call(self.std_streams.as_ref(), self.stack.slice(stack_pos, len));
        for _ in 1..=argc {
            self.stack.pop()?;
        }

        if let Some(val) = res {
            self.stack.push(val);
        }

        Ok(())
    }

    fn current_frame(&self) -> Ref<CUnitFrame> {
        let last_frame = self.frames.retrieve_at(self.current_frame);
        last_frame.borrow()
    }

    fn current_frame_mut(&mut self) -> RefMut<CUnitFrame> {
        let last_frame = self.frames.retrieve_at(self.current_frame);
        last_frame.borrow_mut()
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
            CUnit::Function(funit) => match (funit.ret_type(), val) {
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
    pub(crate) fn new(cunit: CUnit) -> Self {
        Self {
            cunit,
            pointer: 0,
            stack_pos: 0,
        }
    }

    fn inc_pointer(&mut self, by: usize) {
        self.pointer += by;
    }

    fn dec_pointer(&mut self, by: usize) {
        self.pointer -= by;
    }

    fn next(&self) -> Option<OpCode> {
        if self.pointer >= self.cunit.chunk().codes().len() {
            None
        } else {
            Some(self.cunit.chunk().codes()[self.pointer].clone())
        }
    }
}
