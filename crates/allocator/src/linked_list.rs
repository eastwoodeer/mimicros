use core::option::Option::{self, *};
use core::ptr;

#[derive(Copy, Clone)]
pub struct LinkedList {
    head: *mut usize,
}

impl LinkedList {
    pub const fn new() -> Self {
        Self {
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub unsafe fn push(&mut self, item: *mut usize) {
        *item = self.head as usize;
        self.head = item;
    }

    pub fn pop(&mut self) -> Option<*mut usize> {
        match self.is_empty() {
            true => None,
            false => {
                let item = self.head;
                self.head = unsafe { *item as *mut usize };
                Some(item)
            }
        }
    }
}
