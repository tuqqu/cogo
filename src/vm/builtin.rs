#[allow(unused_imports)]
use std::io::Write;

use super::io::StreamProvider;
use super::Vm;
use crate::compiler::{ValType, Value};

pub(super) struct FuncBuiltin {
    name: &'static str,
    argc: Option<u8>,
    func: Builtin,
}

type Builtin = fn(argv: &[Value], streams: &dyn StreamProvider) -> Option<Value>;

impl FuncBuiltin {
    pub(super) fn new(name: &'static str, argc: Option<u8>, func: Builtin) -> Self {
        Self { name, func, argc }
    }

    pub(super) fn call(&self, argv: &[Value], streams: &dyn StreamProvider) -> Option<Value> {
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
    }

    fn define_builtin(&mut self, name: &'static str, argc: Option<u8>, func: Builtin) {
        self.builtins
            .insert(name.to_string(), FuncBuiltin::new(name, argc, func))
            .unwrap_or(());
    }
}

fn builtin_print(argv: &[Value], streams: &dyn StreamProvider) -> Option<Value> {
    write!(
        &mut streams.stream_err(),
        "{}",
        argv.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    )
    .unwrap();
    None
}

fn builtin_println(argv: &[Value], streams: &dyn StreamProvider) -> Option<Value> {
    writeln!(
        &mut streams.stream_err(),
        "{}",
        argv.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    )
    .unwrap();
    None
}

fn builtin_int(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int);
    Some(v)
}

fn builtin_int8(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int8);
    Some(v)
}

fn builtin_int16(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int16);
    Some(v)
}

fn builtin_int32(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int32);
    Some(v)
}

fn builtin_int64(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int64);
    Some(v)
}

fn builtin_uint(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint);
    Some(v)
}

fn builtin_uint8(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint8);
    Some(v)
}

fn builtin_uint16(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint16);
    Some(v)
}

fn builtin_uint32(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint32);
    Some(v)
}

fn builtin_uint64(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint64);
    Some(v)
}

fn builtin_uintptr(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uintptr);
    Some(v)
}

fn builtin_float32(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Float32);
    Some(v)
}

fn builtin_float64(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Float64);
    Some(v)
}

//FIXME
// fn builtin_string(argv: &[Value], _: &dyn StreamProvider) -> Option<Value> {
//     let v = argv.first().unwrap();
//     let v = v.cast_to(ValType::String);
//     Some(v)
// }
