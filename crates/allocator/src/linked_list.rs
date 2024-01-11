use core::fmt;
use core::marker::PhantomData;
use core::marker::Send;
use core::option::Option::{self, *};
use core::ptr;

#[derive(Copy, Clone)]
pub struct LinkedList {
    head: *mut usize,
}

unsafe impl Send for LinkedList {}

pub struct Iter<'a> {
    current: *mut usize,
    list: PhantomData<&'a LinkedList>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = *mut usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            let item = self.current;
            self.current = unsafe { *item as *mut usize };
            Some(item)
        }
    }
}

pub struct ListNode {
    previous: *mut usize,
    current: *mut usize,
}

impl ListNode {
    pub fn pop_current(self) -> *mut usize {
        unsafe {
            *self.previous = *self.current;
        }

        self.current
    }

    pub fn value(&self) -> *mut usize {
        self.current
    }
}

pub struct IterMut<'a> {
    previous: *mut usize,
    current: *mut usize,
    list: PhantomData<&'a mut LinkedList>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = ListNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() {
            None
        } else {
            let node = ListNode {
                previous: self.previous,
                current: self.current,
            };

            self.previous = self.current;
            self.current = unsafe { *self.previous as *mut usize };

            Some(node)
        }
    }
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

    pub fn iter(&self) -> Iter {
        Iter {
            current: self.head,
            list: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut {
        IterMut {
            previous: &mut self.head as *mut *mut usize as *mut usize,
            current: self.head,
            list: PhantomData,
        }
    }
}

impl fmt::Debug for LinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.iter()).finish()
    }
}
