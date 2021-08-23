#[derive(Debug)]
pub(super) struct Scope {
    pub(super) vars: Vec<Local>,
    pub(super) depth: usize,
}

impl Scope {
    pub(super) fn new() -> Self {
        Self {
            vars: Vec::new(),
            depth: 0,
        }
    }

    pub(super) fn add_var(&mut self, name: String) {
        self.vars.push(Local {
            name,
            depth: -1,
            mutable: true,
        });
    }

    pub(super) fn add_const(&mut self, name: String) {
        self.vars.push(Local {
            name,
            depth: -1,
            mutable: false,
        });
    }

    pub(super) fn has_defined(&self, name: &str) -> bool {
        for var in &self.vars {
            if var.depth != -1 && var.depth < self.depth as isize {
                break;
            } else if name == var.name {
                return true;
            }
        }

        false
    }

    pub(super) fn resolve(&self, name: &str) -> Option<(usize, bool)> {
        //FIXME fix the var x = x scoped problem
        for i in (0..self.vars.len()).rev() {
            if self.vars[i].name == *name {
                return Some((i, self.vars[i].mutable));
            }
        }

        None
    }

    pub(super) fn init_last(&mut self) {
        if self.depth == 0 {
            return;
        }

        if let Some(mut last) = self.vars.pop() {
            last.depth = self.depth as isize;
            self.vars.push(last);
        }
    }
}

#[derive(Debug)]
pub(super) struct Local {
    pub(super) name: String,
    pub(super) mutable: bool,
    pub(super) depth: isize,
}
