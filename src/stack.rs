#[derive(Debug, Clone, Default)]
pub struct Stack {
    stack: Vec<i64>,
}

impl Stack {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    pub fn push_front(&mut self, itm: i64) {
        self.stack.insert(0, itm);
    }

    pub fn push_back(&mut self, itm: i64) {
        self.stack.push(itm);
    }

    pub fn pop_front(&mut self) -> Option<i64> {
        if self.stack.is_empty() { return None; }

        Some(self.stack.remove(0))
    }

    pub fn pop_back(&mut self) -> Option<i64> {
        self.stack.pop()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn get_raw_stack(&self) -> &Vec<i64> {
        &self.stack
    }

    pub fn get_raw_stack_mut(&mut self) -> &mut Vec<i64> {
        &mut self.stack
    }

    pub fn peek_back(&self) -> Option<i64> {
        self.stack.last().copied()
    }

    pub fn peek_front(&self) -> Option<i64> {
        self.stack.first().copied()
    }
}
