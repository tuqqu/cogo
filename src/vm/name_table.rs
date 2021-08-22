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
pub struct NameTable<N> {
    funcs: HashMap<String, N>,
}

pub struct NameError(pub(crate) String);

pub type NameResult<T> = result::Result<T, NameError>;

impl<N> NameTable<N> {
    pub fn new() -> Self {
        Self {
            funcs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: String, func: N) -> NameResult<()> {
        if self.funcs.contains_key(&name) {
            return Err(NameError(format!("Name {} already exists", name)));
        }

        self.funcs.insert(name, func);

        Ok(())
    }

    pub fn get(&mut self, name: &str) -> NameResult<&N> {
        let func = self.funcs.get(name);
        if let Some(func) = func {
            Ok(func)
        } else {
            Err(NameError(format!("No name {} exists", name)))
        }
    }
}
