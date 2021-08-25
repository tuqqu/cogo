use std::collections::HashMap;
use std::result;

// use std::sync::atomic::{AtomicUsize, Ordering};

// pub type NameId = usize;
//
// static ATOMIC_COUNTER: AtomicUsize = AtomicUsize::new(1);
//
// fn next_id() -> NameId {
//     ATOMIC_COUNTER.fetch_add(1, Ordering::Relaxed)
// }

#[derive(Debug)]
pub(super) struct NameTable<N>(HashMap<String, N>);

pub(super) struct NameError(pub(crate) String);

type NameResult<T> = result::Result<T, NameError>;

impl<N> NameTable<N> {
    pub(super) fn new() -> Self {
        Self(HashMap::new())
    }

    pub(super) fn insert(&mut self, name: String, value: N) -> NameResult<()> {
        if self.0.contains_key(&name) {
            return Err(NameError(format!("Name {} already exists", name)));
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
}
