use std::result;

#[derive(Debug)]
pub(crate) struct VmStack<T> {
    stack: Vec<T>,
}

#[derive(Debug)]
pub(crate) struct StackUnderflow;
type PopResult<T> = result::Result<T, StackUnderflow>;

impl<T> VmStack<T> {
    pub(super) fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub(super) fn push(&mut self, v: T) {
        self.stack.push(v);
    }

    pub(super) fn pop(&mut self) -> PopResult<T> {
        if let Some(v) = self.stack.pop() {
            Ok(v)
        } else {
            Err(StackUnderflow)
        }
    }

    pub(super) fn retrieve(&self) -> &T {
        self.retrieve_at(self.stack.len() - 1)
    }

    pub(super) fn retrieve_mut(&mut self) -> &mut T {
        self.retrieve_at_mut(self.stack.len() - 1)
    }

    pub(super) fn retrieve_at(&self, i: usize) -> &T {
        self.stack
            .get(i)
            .expect("Cannot retrieve value from stack.")
    }

    pub(super) fn retrieve_at_mut(&mut self, i: usize) -> &mut T {
        self.stack
            .get_mut(i)
            .expect("Cannot retrieve value from stack.")
    }

    pub(super) fn retrieve_by(&self, by: usize) -> &T {
        self.retrieve_at(self.stack.len() - by - 1)
    }

    pub(super) fn retrieve_by_mut(&mut self, by: usize) -> &mut T {
        self.retrieve_at_mut(self.stack.len() - by - 1)
    }

    pub(super) fn last_mut(&mut self) -> &mut T {
        self.stack
            .last_mut()
            .expect("Cannot retrieve mutable reference on an empty stack.")
    }

    pub(super) fn put_at(&mut self, i: usize, v: T) {
        self.stack[i] = v;
    }

    pub(super) fn len(&self) -> usize {
        self.stack.len()
    }

    pub(super) fn slice(&mut self, s: usize, e: usize) -> &[T] {
        &mut self.stack[s..e]
    }
}
