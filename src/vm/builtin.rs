use crate::compiler::Value;
use crate::vm::name_table::NameTable;

pub(super) struct FuncBuiltin {
    name: String,
    func: Builtin,
}

type Builtin = fn(argv: &mut [Value]) -> Option<Value>;

impl FuncBuiltin {
    pub(super) fn new(name: String, func: Builtin) -> Self {
        Self { name, func }
    }

    pub(super) fn call(&self, argv: &mut [Value]) -> Option<Value> {
        (self.func)(argv)
    }

    pub(super) fn name(&self) -> &str {
        &self.name
    }
}

pub(super) fn define_builtin(table: &mut NameTable<FuncBuiltin>, func: FuncBuiltin) {
    table.insert(func.name.clone(), func).unwrap_or(());
}

pub(super) fn builtin_print(argv: &mut [Value]) -> Option<Value> {
    eprint!(
        "{}",
        argv.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
    None
}

pub(super) fn builtin_println(argv: &mut [Value]) -> Option<Value> {
    eprintln!(
        "{}",
        argv.iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
    None
}
