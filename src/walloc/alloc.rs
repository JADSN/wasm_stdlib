use crate::errors::ErrorKind;
use crate::mem::PAGE_SIZE;

pub struct Box {
    memory: [u8; PAGE_SIZE],
    length: usize,
    capacity: usize,
}

impl Default for Box {
    fn default() -> Self {
        Self::new()
    }
}

impl Box {
    pub fn new() -> Self {
        Self {
            memory: [0; PAGE_SIZE],
            length: 0,
            capacity: PAGE_SIZE,
        }
    }

    pub fn from(msg: &str) -> Self {
        let mut vec = Self {
            memory: [0; PAGE_SIZE],
            length: 0,
            capacity: PAGE_SIZE,
        };
        for byte in msg.as_bytes() {
           let _ = vec.push(*byte);
        }
        vec
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

    pub fn get(&self, idx: usize) -> Result<u8, ErrorKind> {
        if idx < self.len() {
            let byte = *self.memory.get(idx).unwrap();
            Ok(byte)
        } else {
            Err(ErrorKind::Overflow)
        }
    }

    pub fn delete(&mut self, idx: usize) -> Result<usize, ErrorKind> {
        if !self.is_empty() {
            let last_idx = self.len() - 1;
            if idx == last_idx {
                self.pop();
            } else {
                for i in idx..(last_idx) {
                    let next_idx = i + 1;
                    self.memory[i] = self.memory[next_idx];
                    self.memory[next_idx] = 0;
                }
                self.length -= 1;
            }
            Ok(self.len())
        } else {
            Err(ErrorKind::NoData)
        }
    }

}
