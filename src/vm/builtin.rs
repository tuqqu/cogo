#[allow(unused_imports)]
use std::io::Write;
use std::result;

use super::error::VmError;
use super::io::StreamProvider;
use super::Vm;
use crate::compiler::{ValType, Value};

pub(super) struct FuncBuiltin {
    name: &'static str,
    argc: Option<u8>,
    func: Builtin,
}

type Builtin = fn(argv: &[Value], streams: &dyn StreamProvider) -> CallResult;
pub(super) type CallResult = result::Result<Option<Value>, VmError>;

impl FuncBuiltin {
    pub(super) fn new(name: &'static str, argc: Option<u8>, func: Builtin) -> Self {
        Self { name, func, argc }
    }

    pub(super) fn call(&self, argv: &[Value], streams: &dyn StreamProvider) -> CallResult {
        (self.func)(argv, streams)
    }

    pub(super) fn name(&self) -> &str {
        self.name
    }

    pub(super) fn argc(&self) -> &Option<u8> {
        &self.argc
    }
}

impl Vm {
    pub(super) fn define_builtins(&mut self) {
        self.define_builtin("print", None, builtin_print);
        self.define_builtin("println", None, builtin_println);
        self.define_builtin("int", Some(1), builtin_int);
        self.define_builtin("int8", Some(1), builtin_int8);
        self.define_builtin("int16", Some(1), builtin_int16);
        self.define_builtin("int32", Some(1), builtin_int32);
        self.define_builtin("int64", Some(1), builtin_int64);
        self.define_builtin("uint", Some(1), builtin_uint);
        self.define_builtin("uintptr", Some(1), builtin_uintptr);
        self.define_builtin("uint8", Some(1), builtin_uint8);
        self.define_builtin("uint16", Some(1), builtin_uint16);
        self.define_builtin("uint32", Some(1), builtin_uint32);
        self.define_builtin("uint64", Some(1), builtin_uint64);
        self.define_builtin("float32", Some(1), builtin_float32);
        self.define_builtin("float64", Some(1), builtin_float64);
        self.define_builtin("len", Some(1), builtin_len);
        self.define_builtin("append", None, builtin_append);
    }

    fn define_builtin(&mut self, name: &'static str, argc: Option<u8>, func: Builtin) {
        self.builtins
            .insert(name.to_string(), FuncBuiltin::new(name, argc, func))
            .unwrap_or(());
    }
}

fn builtin_print(argv: &[Value], streams: &dyn StreamProvider) -> CallResult {
    write!(
        &mut streams.stream_err(),
        "{}",
        argv.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    )
    .unwrap();
    Ok(None)
}

fn builtin_println(argv: &[Value], streams: &dyn StreamProvider) -> CallResult {
    writeln!(
        &mut streams.stream_err(),
        "{}",
        argv.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();
    Ok(None)
}

fn builtin_int(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int);
    Ok(Some(v))
}

fn builtin_int8(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int8);
    Ok(Some(v))
}

fn builtin_int16(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int16);
    Ok(Some(v))
}

fn builtin_int32(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int32);
    Ok(Some(v))
}

fn builtin_int64(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int64);
    Ok(Some(v))
}

fn builtin_uint(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint);
    Ok(Some(v))
}

fn builtin_uint8(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint8);
    Ok(Some(v))
}

fn builtin_uint16(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint16);
    Ok(Some(v))
}

fn builtin_uint32(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint32);
    Ok(Some(v))
}

fn builtin_uint64(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint64);
    Ok(Some(v))
}

fn builtin_uintptr(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uintptr);
    Ok(Some(v))
}

fn builtin_float32(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Float32);
    Ok(Some(v))
}

fn builtin_float64(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Float64);
    Ok(Some(v))
}

//FIXME
// fn builtin_string(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
//     let v = argv.first().unwrap();
//     let v = v.cast_to(ValType::String);
//     Some(v)
// }

fn builtin_len(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let len = match v {
        Value::String(v) => v.len(),
        Value::Array(_, size, _) => *size,
        Value::Slice(iter, _) => iter.borrow().len(),
        _ => {
            return Err(VmError::Compile(format!(
                "Invalid argument type {}",
                v.get_type().name()
            )))
        }
    };

    // go specification dictates type int, not uint
    Ok(Some(Value::Int(len as isize)))
}

fn builtin_append(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    if let Value::Slice(slice, ValType::Slice(vtype)) = v {
        for arg in argv.iter().skip(1) {
            if !arg.is_of_type(vtype) {
                return Err(VmError::Compile(format!(
                    "Expected values of type {} to append, got {}",
                    vtype,
                    arg.get_type().name()
                )));
            }

            slice.borrow_mut().push(arg.clone());
        }
    } else {
        return Err(VmError::Compile(format!(
            "Expected slice as the first argument, got {}",
            v.get_type().name()
        )));
    }

    Ok(Some(v.clone()))
}
