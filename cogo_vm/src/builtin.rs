#[allow(unused_imports)]
use std::io::Write;
use std::{char, result};

use cogo_compiler::{ValType, Value};

use crate::error::VmError;
use crate::io::StreamProvider;
use crate::Vm;

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
        self.define_builtin("string", Some(1), builtin_string);
        self.define_builtin("len", Some(1), builtin_len);
        self.define_builtin("append", None, builtin_append);
        self.define_builtin("complex", Some(2), builtin_complex);
        self.define_builtin("real", Some(1), builtin_real);
        self.define_builtin("imag", Some(1), builtin_imag);
        self.define_builtin("panic", Some(1), builtin_panic);
    }

    fn define_builtin(&mut self, name: &'static str, argc: Option<u8>, func: Builtin) {
        self.builtins
            .insert(name.to_string(), FuncBuiltin::new(name, argc, func))
            .unwrap_or(());
    }
}

/// https://pkg.go.dev/builtin#print
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

/// https://pkg.go.dev/builtin#println
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

/// https://pkg.go.dev/builtin#int
fn builtin_int(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#int8
fn builtin_int8(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int8);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#int16
fn builtin_int16(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int16);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#int32
fn builtin_int32(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int32);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#int64
fn builtin_int64(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Int64);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#uint
fn builtin_uint(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#uint8
fn builtin_uint8(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint8);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#uint16
fn builtin_uint16(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint16);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#uint32
fn builtin_uint32(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint32);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#uint64
fn builtin_uint64(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uint64);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#uintptr
fn builtin_uintptr(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Uintptr);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#float32
fn builtin_float32(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Float32);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#float64
fn builtin_float64(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = v.cast_to(ValType::Float64);
    Ok(Some(v))
}

/// https://pkg.go.dev/builtin#string
fn builtin_string(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let v = match v {
        Value::String(v) => v.clone(),
        Value::Slice(iter, ValType::Slice(vtype))
            if matches!(**vtype, ValType::Int32 | ValType::Uint8) =>
        {
            let chars: String = iter
                .as_ref()
                .borrow()
                .iter()
                .map(|v| unsafe { char::from_u32_unchecked(v.to_usize().unwrap() as u32) })
                .collect();
            chars
        }
        _ => {
            if let Some(int) = v.to_usize() {
                unsafe { char::from_u32_unchecked(int as u32).to_string() }
            } else {
                return Err(VmError::invalid_argument(
                    "uint8 slice, int32 slice, string or integer",
                    &v.get_type(),
                    1,
                ));
            }
        }
    };

    Ok(Some(Value::String(v)))
}

/// https://pkg.go.dev/builtin#len
fn builtin_len(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    let len = match v {
        Value::String(v) => v.len(),
        Value::Array(_, size, _) => *size,
        Value::Slice(iter, _) => iter.borrow().len(),
        _ => {
            return Err(VmError::invalid_argument(
                "string, array, slice",
                &v.get_type(),
                1,
            ));
        }
    };

    // go specification dictates type int, not uint
    Ok(Some(Value::Int(len as isize)))
}

/// https://pkg.go.dev/builtin#append
fn builtin_append(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    if let Value::Slice(slice, ValType::Slice(vtype)) = v {
        for (i, arg) in argv.iter().skip(1).enumerate() {
            if !arg.is_of_type(vtype) {
                return Err(VmError::invalid_argument(vtype, &arg.get_type(), i as u8));
            }

            slice.borrow_mut().push(arg.clone());
        }
    } else {
        return Err(VmError::invalid_argument("slice", &v.get_type(), 1));
    }

    Ok(Some(v.clone()))
}

/// https://pkg.go.dev/builtin#complex
fn builtin_complex(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    match (&argv[0], &argv[1]) {
        (Value::Float32(real), Value::Float32(imag)) => Ok(Some(Value::Complex64(*real, *imag))),
        (Value::Float64(real), Value::Float64(imag)) => Ok(Some(Value::Complex128(*real, *imag))),
        (real @ Value::Float32(_), imag) | (real @ Value::Float64(_), imag) => Err(
            VmError::invalid_argument(&real.get_type(), &imag.get_type(), 1),
        ),
        (real, _) => Err(VmError::invalid_argument("float", &real.get_type(), 0)),
    }
}

/// https://pkg.go.dev/builtin#imag
fn builtin_imag(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    match v {
        Value::Complex64(_, imag) => Ok(Some(Value::Float32(*imag))),
        Value::Complex128(_, imag) => Ok(Some(Value::Float64(*imag))),
        _ => Err(VmError::invalid_argument("complex", &v.get_type(), 0)),
    }
}

/// https://pkg.go.dev/builtin#real
fn builtin_real(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    match v {
        Value::Complex64(real, _) => Ok(Some(Value::Float32(*real))),
        Value::Complex128(real, _) => Ok(Some(Value::Float64(*real))),
        _ => Err(VmError::invalid_argument("complex", &v.get_type(), 0)),
    }
}

/// https://pkg.go.dev/builtin#panic
fn builtin_panic(argv: &[Value], _: &dyn StreamProvider) -> CallResult {
    let v = argv.first().unwrap();
    panic!("{}", v)
}
