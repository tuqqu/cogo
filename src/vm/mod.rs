use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;
use std::result;

use self::builtin::FuncBuiltin;
use self::error::VmError;
use self::io::{StdStreamProvider, StreamProvider};
use self::name_table::NameTable;
use self::stack::VmStack;
use crate::compiler::unit::{CompilationUnit as CUnit, FuncUnit};
use crate::compiler::{OpCode, ValType, Value};

mod builtin;
mod error;
pub mod io;
mod name_table;
mod stack;

#[derive(Debug)]
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

pub type VmResult<T> = result::Result<T, VmError>;
type VmRuntimeCall<T> = std::result::Result<T, VmError>;

pub struct Vm {
    globals: NameTable<VmNamedValue>,
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
            globals: NameTable::new(),
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

    pub fn run(&mut self) -> VmResult<()> {
        let mut match_val: Option<Value> = None;
        let mut switches: VmStack<Switch> = VmStack::new();
        let mut last_call: Call = Call::new(0, false);
        let mut ignore_next_pop = false;

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
                OpCode::Return(len) => {
                    let mut vals: Vec<Value> = vec![];
                    if len != 0 {
                        for _ in 0..len {
                            vals.push(self.stack.pop()?)
                        }
                    }

                    self.validate_return_type(&vals)?;
                    self.discard_frame_stack()?;
                    self.current_frame -= 1;
                    self.frames.pop()?;

                    if len != 0 {
                        for val in vals {
                            self.stack.push(val);
                        }
                    }
                    // we don't want to increment the frame pointer
                    continue;
                }
                OpCode::Bool(v) | OpCode::String(v) => {
                    self.stack.push(v);
                }
                OpCode::IntLiteral(v) | OpCode::FloatLiteral(v) => {
                    self.stack.push(v);
                }
                OpCode::Func(funit) => {
                    if let CUnit::Function(func) = funit {
                        let func_name = func.function().0.to_string();
                        self.names.insert(func_name.clone(), func)?;
                        self.stack.push(Value::Func(func_name));
                    } else {
                        error::panic_at_cunit_type(&funit);
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
                    if !ignore_next_pop {
                        self.stack.pop()?;
                    } else {
                        ignore_next_pop = false;
                    }
                }
                OpCode::VarGlobal(name, vtype) => {
                    let mut value = self.stack.pop()?;
                    if let Some(vtype) = &vtype {
                        if !value.is_of_type(vtype) {
                            return Err(VmError::type_error(vtype, &value.get_type()));
                        }
                        value.lose_literal(vtype);
                    } else {
                        value.lose_literal_blindly();
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Var(value))?;
                }
                OpCode::ConstGlobal(name, vtype) => {
                    let mut value = self.stack.pop()?;
                    if let Some(vtype) = &vtype {
                        if !value.is_of_type(vtype) {
                            return Err(VmError::type_error(vtype, &value.get_type()));
                        }
                        value.lose_literal(vtype);
                    } else {
                        value.lose_literal_blindly();
                    }

                    self.globals
                        .insert(name.clone(), VmNamedValue::Const(value))?;
                }
                OpCode::GetGlobal(name) => {
                    if let Ok(nval) = self.globals.get(&name) {
                        self.stack.push(nval.val().clone());
                    } else if let Ok(builtin) = self.builtins.get(&name) {
                        let val = Value::FuncBuiltin(builtin.name().to_string());
                        self.stack.push(val);
                    } else {
                        return Err(VmError::undefined(&name));
                    }
                }
                OpCode::SetGlobal(name) => {
                    if !self.globals.has(&name) {
                        return Err(VmError::undefined(&name));
                    }

                    let mut value = self.stack.pop()?;
                    value.copy_if_soft_reference();

                    let old_v = self.globals.get_mut(&name)?;
                    if let VmNamedValue::Const(_) = old_v {
                        return Err(VmError::assignment(&name));
                    }

                    let old_v = old_v.val_mut();
                    // FIXME: maybe we should store types in a sep hashtable?
                    if !old_v.same_type(&value) {
                        return Err(VmError::type_error(&old_v.get_type(), &value.get_type()));
                    }

                    value.lose_literal(&old_v.get_type());
                    *old_v = value.clone();
                    ignore_next_pop = true;
                }
                OpCode::LoseSoftReference(by) => {
                    let value = self.stack.retrieve_by_mut(by);
                    value.copy_if_soft_reference();
                }
                OpCode::GetLocal(i) => {
                    let offset = self.current_frame().stack_pos;
                    let mut value = self.stack.retrieve_at(i + offset).clone();
                    value.lose_literal_blindly();
                    self.stack.push(value);
                }
                OpCode::SetLocal(i) => {
                    let offset = self.current_frame().stack_pos;
                    let stack_pos = i + offset;

                    let old_v = self.stack.retrieve_at(stack_pos).clone();
                    let mut value = self.stack.pop()?;
                    value.lose_literal(&old_v.get_type());
                    value.copy_if_soft_reference();

                    if !old_v.same_type(&value) {
                        return Err(VmError::type_error(&old_v.get_type(), &value.get_type()));
                    }

                    self.stack.put_at(stack_pos, value);
                    ignore_next_pop = true;
                }
                OpCode::Call(argc, spread) => {
                    last_call = Call::new(argc, spread);
                    let val = self.stack.retrieve_by(argc as usize).clone();
                    match val {
                        Value::Func(name) => {
                            for arg in 0..argc {
                                let arg = self.stack.retrieve_by_mut(arg as usize);
                                arg.copy_if_soft_reference();
                            }

                            self.call_func(&name, argc, spread)?;
                            self.current_frame_mut().inc_pointer(1);
                            self.current_frame += 1;
                            continue;
                        }
                        Value::FuncBuiltin(name) => {
                            self.call_builtin(&name, argc)?;
                        }
                        _ => {
                            return Err(VmError::callable_value_expected(&val.get_type()));
                        }
                    }
                }
                OpCode::GetIndex => {
                    let index = self.stack.pop()?;
                    let index = iter_utils::unwrap_index(index)?;
                    let iter = self.stack.pop()?;

                    self.stack.push(iter_utils::get_at_index(&iter, index)?);
                }
                OpCode::GetLocalIndex(i) => {
                    let index = self.stack.pop()?;
                    let index = iter_utils::unwrap_index(index)?;

                    let offset = self.current_frame().stack_pos;
                    let iter = self.stack.retrieve_at(i + offset).clone();

                    self.stack.push(iter_utils::get_at_index(&iter, index)?);
                }
                OpCode::GetGlobalIndex(name) => {
                    let index = self.stack.pop()?;
                    let index = iter_utils::unwrap_index(index)?;

                    let iter = self.globals.get(&name)?.val();

                    self.stack.push(iter_utils::get_at_index(iter, index)?);
                }
                OpCode::SetIndex => {
                    let value = self.stack.pop()?;
                    let index = self.stack.pop()?;
                    let index = iter_utils::unwrap_index(index)?;

                    let mut iter = self.stack.pop()?;
                    iter_utils::set_at_index(&mut iter, index, value)?;
                    ignore_next_pop = true;
                }
                OpCode::SetLocalIndex(i, index_at, array_at_index) => {
                    let value = self.stack.pop()?;
                    let index = self.stack.pop_at(self.stack.len() - index_at);
                    let index = iter_utils::unwrap_index(index)?;

                    let mut iter = if array_at_index {
                        self.stack.pop_at(self.stack.len() - index_at)
                    } else {
                        let offset = self.current_frame().stack_pos;
                        self.stack.retrieve_at(i + offset).clone()
                    };

                    iter_utils::set_at_index(&mut iter, index, value)?;
                    ignore_next_pop = true;
                }
                OpCode::SetGlobalIndex(name, index_at, array_at_index) => {
                    let value = self.stack.pop()?;
                    let index = self.stack.pop_at(self.stack.len() - index_at);
                    let index = iter_utils::unwrap_index(index)?;

                    let mut iter = if array_at_index {
                        self.stack.pop_at(self.stack.len() - index_at)
                    } else {
                        self.globals.get_mut(&name)?.val_mut().clone()
                    };

                    iter_utils::set_at_index(&mut iter, index, value)?;
                    ignore_next_pop = true;
                }
                OpCode::BlindLiteralCast(by) => {
                    let val = self.stack.retrieve_by_mut(by);
                    val.lose_literal_blindly();
                }
                OpCode::ArrayLiteral(size, array_type) => {
                    let mut vals = vec![];
                    if let ValType::Array(vtype, type_size) = &array_type {
                        if *type_size != size {
                            return Err(VmError::wrong_array_size(*type_size, size));
                        }

                        for _ in 0..size {
                            let mut val = self.stack.pop()?;
                            val.lose_literal(vtype);
                            if !val.is_of_type(vtype) {
                                return Err(VmError::type_error(vtype, &val.get_type()));
                            }
                            vals.push(val);
                        }

                        vals.reverse();
                        self.stack.push(Value::new_array(vals, size, array_type));
                    } else {
                        return Err(VmError::incorrectly_typed("array literal", &array_type));
                    }
                }
                OpCode::SliceLiteral(size, slice_type) => {
                    let mut vals = vec![];
                    if let ValType::Slice(vtype) = &slice_type {
                        for _ in 0..size {
                            let mut val = self.stack.pop()?;
                            val.lose_literal(vtype);
                            if !val.is_of_type(vtype) {
                                return Err(VmError::type_error(vtype, &val.get_type()));
                            }
                            vals.push(val);
                        }

                        vals.reverse();
                        self.stack.push(Value::new_slice(vals, slice_type));
                    } else {
                        return Err(VmError::incorrectly_typed("slice literal", &slice_type));
                    }
                }
                OpCode::TypeValidation(vtype, at) => {
                    let val = self.stack.retrieve_by_mut(at);
                    val.lose_literal(&vtype);
                    if !val.is_of_type(&vtype) {
                        return Err(VmError::type_error(&vtype, &val.get_type()));
                    }
                }
                OpCode::VariadicSliceCast(vtype, until) => {
                    if !last_call.spread {
                        let length = last_call.argc - until;
                        let mut slice = Vec::<Value>::with_capacity(length as usize);
                        for _ in 0..length {
                            let val = self.stack.pop()?;
                            if !val.is_of_type(&vtype) {
                                return Err(VmError::type_error(&vtype, &val.get_type()));
                            }
                            slice.push(val);
                        }
                        slice.reverse();

                        let slice = Value::new_slice(slice, vtype);
                        self.stack.push(slice);
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
                            return Err(VmError::non_bool_in_condition(&val.get_type()));
                        }
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
                                _ => return Err(VmError::unexpected_matching_result()),
                            }
                        } else {
                            return Err(VmError::non_exhaustive_matching_result());
                        }
                    } else {
                        last.fall_flag = false;
                    }
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
            }

            self.current_frame_mut().inc_pointer(1);
        }

        VmResult::Ok(())
    }

    fn call_func(&mut self, name: &str, argc: u8, spread: bool) -> VmRuntimeCall<()> {
        let f = self.names.get(name)?;
        if (!f.is_variadic() || f.is_variadic() && spread) && argc as usize != f.argc() {
            return Err(VmError::mismatched_argc(f.argc(), argc));
        }

        let mut frame = CUnitFrame::new(CUnit::Function(f.clone()));
        frame.stack_pos = self.stack.len() - argc as usize;
        self.frames.push(Rc::new(RefCell::new(frame)));

        Ok(())
    }

    fn call_builtin(&mut self, name: &str, argc: u8) -> VmRuntimeCall<()> {
        let f = self.builtins.get(name)?;
        let len = self.stack.len();
        let stack_pos = len - argc as usize;

        if let Some(fargc) = f.argc() {
            if *fargc != argc {
                return Err(VmError::mismatched_argc(*fargc as usize, argc));
            }
        }

        let res = f.call(self.stack.slice(stack_pos, len), self.std_streams.as_ref())?;
        for _ in 1..=argc {
            self.stack.pop()?;
        }

        if let Some(val) = res {
            self.stack.pop()?;
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

    fn discard_frame_stack(&mut self) -> VmResult<()> {
        if self.stack.len() != 0 {
            while self.stack.len() >= self.current_frame().stack_pos {
                self.stack.pop()?;
            }
        }

        Ok(())
    }

    fn validate_return_type(&self, vals: &[Value]) -> VmResult<()> {
        let cunit = &self.current_frame().cunit;
        if let CUnit::Function(funit) = cunit {
            let ctype = funit.ret_type();
            let type_len = ctype.len();
            let val_len = vals.len();
            if type_len != val_len {
                return Err(VmError::return_count_error(type_len, val_len));
            }

            for (i, vtype) in ctype.types().iter().enumerate() {
                let val = &vals[val_len - i - 1];
                if !val.is_of_type(vtype) {
                    return Err(VmError::return_type_error(vtype, &val.get_type()));
                }
            }

            Ok(())
        } else {
            error::panic_at_cunit_type(cunit);
        }
    }
}

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

struct Call {
    argc: u8,
    spread: bool,
}

impl Call {
    fn new(argc: u8, spread: bool) -> Self {
        Self { argc, spread }
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

mod iter_utils {
    use super::*;

    pub(super) fn unwrap_index(index: Value) -> VmResult<usize> {
        if let Some(index) = index.to_usize() {
            Ok(index)
        } else {
            Err(VmError::index_type_error(&index.get_type()))
        }
    }

    pub(super) fn set_at_index(iter: &mut Value, index: usize, mut value: Value) -> VmResult<()> {
        match iter {
            Value::Array(iter, _, ValType::Array(ref vtype, ..))
            | Value::Slice(iter, ValType::Slice(ref vtype)) => {
                value.lose_literal(vtype);
                if !value.is_of_type(vtype) {
                    return Err(VmError::type_error(vtype, &value.get_type()));
                }
                iter.borrow_mut()[index] = value;

                Ok(())
            }
            _ => Err(VmError::iterator_value_expected(&iter.get_type())),
        }
    }

    pub(super) fn get_at_index(iter: &Value, index: usize) -> VmResult<Value> {
        match iter {
            Value::Array(iter, ..) | Value::Slice(iter, ..) => Ok(iter.borrow_mut()[index].clone()),
            _ => Err(VmError::iterator_value_expected(&iter.get_type())),
        }
    }
}
