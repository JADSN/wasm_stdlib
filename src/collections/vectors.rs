// use crate::{collections::stacks::Stack, errors::ErrorKind};
use crate::errors::ErrorKind;

use crate::mem::PAGE_SIZE;

// TODO: Derive from Stack

pub struct Vector {
    memory: [u8; PAGE_SIZE],
    length: usize,
    capacity: usize,
}

impl Default for Vector {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector {
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

    // pub fn get_bulk(&self, idx: usize, offset: usize) -> Result<Vector, ErrorKind> {
    //     if (idx + offset) < self.len() {
    //         let vector = Vector::new();
    //         for i in idx..=offset {
    //             vector.push(*self.memory.get(i).unwrap());
    //         }
    //         Ok(vector)
    //     } else {
    //         Err(ErrorKind::Overflow)
    //     }
    // }
}

mod tests {
    #[test]
    fn valids_as_stack() {
        use super::Vector;
        let mut vector = Vector::new();
        assert_eq!(vector.len(), 0);
        for i in 1..=10 {
            let _ = vector.push(i);
        }
        assert_eq!(vector.len(), 10);
        for i in (1..=10).rev() {
            assert_eq!(i, vector.pop().unwrap());
        }
        assert!(vector.is_empty());
    }

    #[test]
    fn valids_as_vector_get() {
        use super::Vector;
        let mut vector = Vector::new();
        assert_eq!(vector.len(), 0);
        for i in 1..=10 {
            let _ = vector.push(i);
        }
        if let Ok(one) = vector.get(0) {
            assert_eq!(1, one);
        }
        if let Ok(five) = vector.get(4) {
            assert_eq!(5, five);
        }
        if let Ok(ten) = vector.get(9) {
            assert_eq!(10, ten);
        }
    }

    #[test]
    fn valids_as_vector_delete() {
        use super::Vector;
        let mut vector = Vector::new();
        assert_eq!(vector.len(), 0);
        for i in 1..=10 {
            let _ = vector.push(i);
        }

        if vector.delete(4).is_ok() {
            assert_eq!(9, vector.len()); // size == 9
        };

        if vector.delete(4).is_ok() {
            assert_eq!(8, vector.len()); // size == 8
        };

        if let Ok(eight) = vector.get(5) {
            assert_eq!(8, eight);
        }
    }
}
