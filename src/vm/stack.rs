use std::result;

#[derive(Debug)]
pub(crate) struct VmStack<T> {
    stack: Vec<T>,
}

#[derive(Debug)]
pub struct StackUnderflow;
pub type PopResult<T> = result::Result<T, StackUnderflow>;

impl<T> VmStack<T> {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, v: T) {
        self.stack.push(v);
    }

    pub fn pop(&mut self) -> PopResult<T> {
        if let Some(v) = self.stack.pop() {
            Ok(v)
        } else {
            Err(StackUnderflow)
        }
    }

    pub fn retrieve(&self) -> &T {
        self.retrieve_at(self.stack.len() - 1)
    }

    pub fn retrieve_at(&self, i: usize) -> &T {
        self.stack
            .get(i)
            .expect("Cannot retrieve value from stack.")
    }

    pub fn retrieve_by(&self, by: usize) -> &T {
        self.retrieve_at(self.stack.len() - by - 1)
    }

    pub fn last_mut(&mut self) -> &mut T {
        self.stack
            .last_mut()
            .expect("Cannot retrieve mutable reference on an empty stack.")
    }

    pub fn put_at(&mut self, i: usize, v: T) {
        self.stack[i] = v;
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }
}
