use super::unit::FuncUnit;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Package(pub String);

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Function(pub String);

/// Entry point validation and check.
pub(super) struct EntryPoint {
    package: Package,
    function: Function,
}

pub(super) struct SignatureError(pub String);

type SignValidationResult<T> = std::result::Result<T, SignatureError>;

impl EntryPoint {
    pub(super) fn new(package: Package, function: Function) -> Self {
        Self { package, function }
    }

    pub(super) fn func_name(&self) -> &Function {
        &self.function
    }

    /// Checks that if the function is an entry point in a given package,
    /// it does not violate the required signature.
    pub(super) fn check(&self, pack: &Package, funit: &FuncUnit) -> SignValidationResult<()> {
        if *pack == self.package && *funit.function() == self.function {
            if Self::validate_signature(funit) {
                Ok(())
            } else {
                Err(SignatureError(format!(
                    "Function \"{}\" must not have parameters and a return value",
                    self.function.0,
                )))
            }
        } else {
            Ok(())
        }
    }

    fn validate_signature(funit: &FuncUnit) -> bool {
        funit.ret_type().is_none() && funit.param_names().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::super::vtype::FuncType;
    use super::super::ValType;
    use super::*;

    #[test]
    fn test_entry_point_check() {
        let ep = EntryPoint::new(Package("test".to_string()), Function("test".to_string()));

        // a valid entry point
        let package_a = Package("test".to_string());
        let funit_a = create_funit("test".to_string());
        assert!(ep.check(&package_a, &funit_a).is_ok());

        // just a function
        let package_b = Package("another".to_string());
        assert!(ep.check(&package_b, &funit_a).is_ok());

        // just a function
        let funit_b = create_funit("another".to_string());
        assert!(ep.check(&package_a, &funit_b).is_ok());

        // violates the signature rules
        let funit_c = create_funit_with_ret_type("test".to_string(), ValType::Bool);
        assert!(ep.check(&package_a, &funit_c).is_err());
    }

    fn create_funit(fname: String) -> FuncUnit {
        FuncUnit::new(Some(Function(fname)), FuncType::new(vec![], None), vec![])
    }

    fn create_funit_with_ret_type(fname: String, ret_type: ValType) -> FuncUnit {
        FuncUnit::new(
            Some(Function(fname)),
            FuncType::new(vec![], Some(ret_type)),
            vec![],
        )
    }
}
