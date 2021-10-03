use std::collections::HashMap;

#[derive(Debug)]
pub(super) struct ControlFlow {
    continue_jumps: HashMap<usize, usize>,
    loop_breaks: HashMap<usize, Vec<usize>>,
    switch_breaks: HashMap<usize, Vec<usize>>,
    loop_depth: usize,
    switch_depth: usize,
    break_stack: Vec<BreakState>,
}

#[derive(Debug)]
enum BreakState {
    Switch,
    Loop,
}

impl ControlFlow {
    pub(super) fn new() -> Self {
        Self {
            continue_jumps: HashMap::new(),
            loop_breaks: HashMap::new(),
            switch_breaks: HashMap::new(),
            loop_depth: 0,
            switch_depth: 0,
            break_stack: Vec::new(),
        }
    }

    pub(super) fn enter_switch(&mut self) {
        self.switch_depth += 1;
        self.break_stack.push(BreakState::Switch);
    }

    pub(super) fn leave_switch(&mut self) {
        self.switch_depth -= 1;
        self.break_stack.pop();
    }

    pub(super) fn enter_loop(&mut self) {
        self.loop_depth += 1;
        self.break_stack.push(BreakState::Loop);
    }

    pub(super) fn leave_loop(&mut self) {
        self.loop_depth -= 1;
        self.break_stack.pop();
    }

    pub(super) fn add_break(&mut self, jump: usize) {
        match self.break_stack.last().expect("Cannot get state") {
            BreakState::Loop => self.add_loop_break(jump),
            BreakState::Switch => self.add_switch_break(jump),
        }
    }

    pub(super) fn add_loop_break(&mut self, jump: usize) {
        self.loop_breaks
            .entry(self.loop_depth)
            .or_default()
            .push(jump);
    }

    pub(super) fn add_switch_break(&mut self, jump: usize) {
        self.switch_breaks
            .entry(self.switch_depth)
            .or_default()
            .push(jump);
    }

    pub(super) fn add_continue(&mut self, jump: usize) {
        self.continue_jumps.insert(self.loop_depth, jump);
    }

    pub(super) fn loop_breaks(&mut self) -> &mut Vec<usize> {
        self.loop_breaks.entry(self.loop_depth).or_default()
    }

    pub(super) fn switch_breaks(&mut self) -> &mut Vec<usize> {
        self.switch_breaks.entry(self.switch_depth).or_default()
    }

    pub(super) fn continue_jump(&self) -> usize {
        *self
            .continue_jumps
            .get(&self.loop_depth)
            .expect("No continue jump found")
    }

    pub(super) fn is_breakable(&self) -> bool {
        self.loop_depth != 0 || self.switch_depth != 0
    }

    pub(super) fn is_continuable(&self) -> bool {
        self.loop_depth != 0
    }

    pub(super) fn is_fallthroughable(&self) -> bool {
        self.switch_depth != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_control_flow() {
        let mut cf = ControlFlow::new();
        assert!(!cf.is_breakable());
        assert!(!cf.is_continuable());
        assert!(!cf.is_fallthroughable());

        cf.enter_switch();
        assert!(cf.is_breakable());
        assert!(!cf.is_continuable());
        assert!(cf.is_fallthroughable());

        cf.enter_loop();
        assert!(cf.is_breakable());
        assert!(cf.is_continuable());
        assert!(cf.is_fallthroughable());

        cf.leave_loop();
        cf.leave_switch();
        cf.enter_loop();
        assert!(cf.is_breakable());
        assert!(cf.is_continuable());
        assert!(!cf.is_fallthroughable());
    }
}
