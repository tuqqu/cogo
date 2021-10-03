use crate::compiler::error::DefinitionError;

/// Scope counter and resolver
/// Tracks the depth of a scope we are in
/// Has the notion of all the variables inside the scope
/// Can resolve them and say if they are defined or not
#[derive(Debug)]
pub(super) struct Scope {
    pub(super) vars: Vec<Local>,
    pub(super) depth: usize,
}

type ScopeDefinitionResult<T> = Result<T, DefinitionError>;

impl Scope {
    pub(super) fn new() -> Self {
        Self {
            vars: Vec::new(),
            depth: 0,
        }
    }

    pub(super) fn add_var(&mut self, name: String) {
        self.vars.push(Local::new_var(name));
    }

    pub(super) fn add_const(&mut self, name: String) {
        self.vars.push(Local::new_const(name));
    }

    pub(super) fn has_defined(&self, name: &str) -> bool {
        for var in &self.vars {
            if var.is_initialised() && var.depth < self.depth as isize {
                break;
            } else if name == var.name {
                return true;
            }
        }

        false
    }

    pub(super) fn has_defined_var(&self, name: &str) -> ScopeDefinitionResult<bool> {
        for var in &self.vars {
            if var.is_initialised() && var.depth < self.depth as isize {
                break;
            } else if var.mutable && name == var.name {
                return Ok(true);
            } else if !var.mutable && name == var.name {
                return Err(DefinitionError);
            }
        }

        Ok(false)
    }

    pub(super) fn resolve(&self, name: &str) -> Option<(usize, bool)> {
        for (i, var) in self.vars.iter().rev().enumerate() {
            if var.name == *name {
                return Some((self.vars.len() - i - 1, var.mutable));
            }
        }

        None
    }

    pub(super) fn init_last(&mut self) {
        for var in self.vars.iter_mut().rev() {
            if !var.is_initialised() {
                var.depth = self.depth as isize;
                break;
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct Local {
    pub(super) name: String,
    pub(super) mutable: bool,
    pub(super) depth: isize,
}

impl Local {
    const UNINITIALISED: isize = -1;

    fn new_var(name: String) -> Self {
        Self::new(name, true)
    }

    fn new_const(name: String) -> Self {
        Self::new(name, false)
    }

    fn new(name: String, mutable: bool) -> Self {
        Self {
            name,
            depth: Local::UNINITIALISED,
            mutable,
        }
    }

    fn is_initialised(&self) -> bool {
        self.depth != Self::UNINITIALISED
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_var_defining() {
        let mut scope = Scope::new();
        scope.add_var("a".to_string());

        assert!(scope.has_defined("a"));
        assert!(!scope.has_defined("c"));

        scope.depth += 1;
        scope.add_var("c".to_string());
        assert!(scope.has_defined("c"));
    }

    #[test]
    fn test_var_resolving() {
        let mut scope = Scope::new();
        scope.add_var("a".to_string());
        scope.depth += 1;
        scope.add_const("b".to_string());

        assert_eq!(scope.resolve("a"), Some((0, true)));
        assert_eq!(scope.resolve("b"), Some((1, false)));
    }

    #[test]
    fn test_var_initialising() {
        let mut scope = Scope::new();
        scope.add_var("a".to_string());
        scope.add_var("b".to_string());
        scope.depth += 1;
        scope.init_last();
        scope.add_const("c".to_string());
        scope.depth += 1;
        scope.init_last();

        assert_eq!(
            scope.vars,
            vec![
                Local {
                    name: "a".to_string(),
                    depth: -1,
                    mutable: true
                },
                Local {
                    name: "b".to_string(),
                    depth: 1,
                    mutable: true
                },
                Local {
                    name: "c".to_string(),
                    depth: 2,
                    mutable: false
                },
            ]
        );
    }
}
