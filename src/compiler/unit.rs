use super::opcode::Chunk;
use super::ValType;
use crate::compiler::value::FuncType;

#[derive(Clone, Debug)]
pub enum CompilationUnit {
    Package(PackageUnit),
    Function(FuncUnit),
}

impl CompilationUnit {
    pub(crate) fn chunk(&self) -> &Chunk {
        match &self {
            Self::Package(p) => &p.codes,
            Self::Function(f) => &f.codes,
        }
    }

    pub(crate) fn chunk_mut(&mut self) -> &mut Chunk {
        match self {
            Self::Package(p) => &mut p.codes,
            Self::Function(f) => &mut f.codes,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PackageUnit {
    //FIXME use package object
    name: String,
    codes: Chunk,
}

impl PackageUnit {
    pub(super) fn new() -> Self {
        Self {
            name: "".to_string(),
            codes: Chunk::new(),
        }
    }

    pub(super) fn set_name(&mut self, name: String) {
        self.name = name;
    }
}

impl Default for PackageUnit {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct FuncUnit {
    ftype: FuncType,
    param_names: Vec<String>,
    name: String,
    codes: Chunk,
}

impl FuncUnit {
    pub(super) const MAX_ARGC: u8 = u8::MAX;

    pub(super) fn new(name: Option<String>, ftype: FuncType, param_names: Vec<String>) -> Self {
        Self::from_codes(name, ftype, param_names, Chunk::new())
    }

    fn from_codes(
        name: Option<String>,
        ftype: FuncType,
        param_names: Vec<String>,
        codes: Chunk,
    ) -> Self {
        Self {
            param_names,
            ftype,
            name: name.unwrap_or_else(|| "".to_string()),
            codes,
        }
    }

    pub(crate) fn ret_type(&self) -> &Option<ValType> {
        self.ftype.ret_type()
    }

    pub(crate) fn param_names(&self) -> &[String] {
        &self.param_names
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }
}
