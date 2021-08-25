use super::opcode::Chunk;
use super::ValType;

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
    pub(crate) params: Vec<Param>,
    pub(crate) ret_type: Option<ValType>,
    pub(crate) name: String,
    codes: Chunk,
}

impl FuncUnit {
    pub(super) const MAX_ARGC: u8 = u8::MAX;

    pub(super) fn new(name: Option<String>) -> Self {
        Self::from_codes(name, Chunk::new())
    }

    fn from_codes(name: Option<String>, codes: Chunk) -> Self {
        Self {
            params: vec![],
            ret_type: None,
            name: name.unwrap_or_else(|| "".to_string()),
            codes,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Param {
    name: String,
    v_type: ValType,
}

impl Param {
    pub(super) fn new(name: String, v_type: ValType) -> Self {
        Self { name, v_type }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn v_type(&self) -> &ValType {
        &self.v_type
    }
}
