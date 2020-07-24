use crate::errors::ErrorKind;

use crate::mem::PAGE_SIZE;

pub struct Stack {
    memory: [u8; PAGE_SIZE],
    length: usize,
    capacity: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            memory: [0; PAGE_SIZE],
            length: 0,
            capacity: PAGE_SIZE,
        }
    }
    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn push(&mut self, byte: u8) -> Result<usize, ErrorKind> {
        if self.length < self.capacity {
            self.memory[self.length] = byte;
            self.length += 1;
            Ok(self.len())
        } else {
            Err(ErrorKind::Overflow)
        }
    }

    pub fn pop(&mut self) -> Option<u8> {
        if !self.is_empty() {
            let last_idx = self.length - 1;
            let last_data = self.memory[last_idx];
            self.memory[last_idx] = 0;
            self.length -= 1;
            Some(last_data)
        } else {
            None
        }
    }

    pub fn drop(self) {
        let _ = self;
    }
}

mod tests {

    #[test]
    fn valids() {
        use super::Stack;
        let mut stack = Stack::new();
        assert_eq!(stack.len(), 0);
        for i in 1..=10 {
            let _ = stack.push(i);
        }
        assert_eq!(stack.len(), 10);
        for i in (1..=10).rev() {
            assert_eq!(i, stack.pop().unwrap());
        }
        assert!(stack.is_empty());
    }
}
