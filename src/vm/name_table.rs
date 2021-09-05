use std::collections::HashMap;
use std::result;

#[derive(Debug)]
pub(super) struct NameTable<N>(HashMap<String, N>);

pub(super) struct NameError(pub(crate) String);

type NameResult<T> = result::Result<T, NameError>;

impl<N> NameTable<N> {
    pub(super) fn new() -> Self {
        Self(HashMap::new())
    }

    pub(super) fn has(&self, name: &str) -> bool {
        self.0.contains_key(name)
    }

    pub(super) fn insert(&mut self, name: String, value: N) -> NameResult<()> {
        if self.0.contains_key(&name) {
            return Err(NameError(format!("Name {} already declared", name)));
        }

        self.0.insert(name, value);

        Ok(())
    }

    pub(super) fn get(&mut self, name: &str) -> NameResult<&N> {
        let value = self.0.get(name);
        if let Some(value) = value {
            Ok(value)
        } else {
            Err(NameError(format!("No name {} exists", name)))
        }
    }

    pub(super) fn get_mut(&mut self, name: &str) -> NameResult<&mut N> {
        let value = self.0.get_mut(name);
        if let Some(value) = value {
            Ok(value)
        } else {
            Err(NameError(format!("No name {} exists", name)))
        }
    }
}
