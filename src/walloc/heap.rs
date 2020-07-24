use crate::errors::ErrorKind;
use crate::mem::PAGE_SIZE;

// pub const PAGE_SIZE: usize = 4096; // 4 Kbytes

const PAGES_MAX: usize = 256; // 4096 Bytes * 256 Pages = 1 MByte => 1_048_576

type Page = [u8; PAGE_SIZE];
type Memory = [Page; PAGES_MAX];

#[no_mangle]
// static mut HEAP: [u8; PAGE_SIZE] = [0; PAGE_SIZE];
static mut HEAP: Memory = [[0; PAGE_SIZE]; PAGES_MAX];

// TODO: We must manage heap usage

#[no_mangle]
static mut HEAP_INDEX: [bool; PAGES_MAX] = [false; PAGES_MAX];

enum Status {
    Free,
    Used,
}

impl Copy for Status {}

impl Clone for Status {
    fn clone(&self) -> Status {
        *self
    }
}

pub struct Heap {
    index: [Status; PAGES_MAX],
    length: usize,
    capacity: usize,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            index: [Status::Free; PAGES_MAX],
            length: 0,           // start of memory
            capacity: PAGES_MAX, // end of memory
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn cap(&self) -> usize {
        self.capacity
    }

    fn find_free(&mut self) -> Option<usize> {
        for idx in 0..self.len() {
            if let Status::Free = self.index[idx] {
                return Some(idx);
            }
        }
        None
    }

    pub fn alloc(&mut self) -> Result<usize, ErrorKind> {
        if self.len() < self.capacity {
            // find_free slot and return heap index position
            match self.find_free() {
                Some(heap_idx) => Ok(heap_idx),         //TODO: alloc
                None => Err(ErrorKind::NotImplemented), // TODO: Grow Heap increment length
            }
        } else {
            Err(ErrorKind::Overflow)
        }
    }

    pub fn drop(&mut self, idx: usize) {
        if !self.is_empty() {
            if let Status::Used = self.index[idx] {
                self.index[idx] = Status::Free;
            }
        }
    }
}
