#[allow(unused_imports)]
use std::io::Write;

use crate::compiler::Value;
use crate::vm::io::StreamProvider;
use crate::vm::Vm;

pub(super) struct FuncBuiltin {
    name: String,
    func: Builtin,
}

type Builtin = fn(streams: &dyn StreamProvider, argv: &[Value]) -> Option<Value>;

impl FuncBuiltin {
    pub(super) fn new(name: String, func: Builtin) -> Self {
        Self { name, func }
    }

    pub(super) fn call(&self, streams: &dyn StreamProvider, argv: &[Value]) -> Option<Value> {
        (self.func)(streams, argv)
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

impl Vm {
    pub(super) fn define_builtins(&mut self) {
        self.define_builtin(FuncBuiltin::new("print".to_string(), builtin_print));
        self.define_builtin(FuncBuiltin::new("println".to_string(), builtin_println));
    }

    fn define_builtin(&mut self, func: FuncBuiltin) {
        self.builtins.insert(func.name.clone(), func).unwrap_or(());
    }
}

fn builtin_print(streams: &dyn StreamProvider, argv: &[Value]) -> Option<Value> {
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

fn builtin_println(streams: &dyn StreamProvider, argv: &[Value]) -> Option<Value> {
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
