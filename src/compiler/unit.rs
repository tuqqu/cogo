use super::opcode::Chunk;
use super::structure::{Function as FunctionItem, Package as PackageItem};
use super::vtype::{CompositeType, FuncType};

/// Chunk of Opcodes which belongs to either a function or a package
#[derive(Clone, Debug)]
pub enum CompilationUnit {
    Package(PackageUnit),
    Function(FuncUnit),
}

impl CompilationUnit {
    const PACKAGE: &'static str = "package";
    const FUNCTION: &'static str = "function";

    /// Used to output nicer compilation failure messages
    pub(crate) fn cunit_type(&self) -> &str {
        match &self {
            Self::Package(_) => Self::PACKAGE,
            Self::Function(_) => Self::FUNCTION,
        }
    }

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
    package: PackageItem,
    codes: Chunk,
}

impl PackageUnit {
    pub(super) fn new() -> Self {
        Self {
            package: PackageItem("".to_string()),
            codes: Chunk::new(),
        }
    }

    pub(super) fn set_package(&mut self, package: PackageItem) {
        self.package = package;
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
    function: FunctionItem,
    codes: Chunk,
}

impl FuncUnit {
    pub(super) const MAX_ARGC: u8 = u8::MAX;

    pub(super) fn new(name: Option<FunctionItem>, ftype: FuncType) -> Self {
        Self::from_codes(name, ftype, Chunk::new())
    }

    fn from_codes(name: Option<FunctionItem>, ftype: FuncType, codes: Chunk) -> Self {
        Self {
            ftype,
            function: name.unwrap_or_else(|| FunctionItem("".to_string())),
            codes,
        }
    }

    pub(crate) fn ret_type(&self) -> &CompositeType {
        self.ftype.ret_type()
    }

    pub(crate) fn argc(&self) -> usize {
        self.ftype.args().len()
    }

    pub(crate) fn is_variadic(&self) -> bool {
        self.ftype.variadic()
    }

    pub(crate) fn function(&self) -> &FunctionItem {
        &self.function
    }
}
