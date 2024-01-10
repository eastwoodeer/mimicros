use crate::linked_list;

use core::alloc::Layout;
use core::assert;
use core::cmp::{max, min};
use core::fmt;
use core::mem::size_of;
use core::option::Option::Some;
use core::ptr::NonNull;
use core::result::Result::{self, Err, Ok};

pub struct Heap<const ORDER: usize> {
    pub free_list: [linked_list::LinkedList; ORDER],
    allocated: usize,
    total: usize,
}

pub fn prev_power_of_two(n: usize) -> usize {
    1 << (usize::BITS - n.leading_zeros() - 1)
}

impl<const ORDER: usize> Heap<ORDER> {
    pub const fn new() -> Self {
        Self {
            free_list: [linked_list::LinkedList::new(); ORDER],
            allocated: 0,
            total: 0,
        }
    }

    pub fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
        start = (start + size_of::<usize>() - 1) & (!size_of::<usize>() + 1);
        end &= !size_of::<usize>() + 1;
        assert!(start <= end);

        let mut total = 0;
        let mut current_start = start;

        while current_start + size_of::<usize>() <= end {
            let lowbit = current_start & (!current_start + 1);
            let size = min(lowbit, prev_power_of_two(end - current_start));

            unsafe {
                self.free_list[size.trailing_zeros() as usize].push(current_start as *mut usize);
            }

            current_start += size;
            total += size;
        }

        self.total = total;
    }

    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );

        let class = size.trailing_zeros() as usize;
        for i in class..self.free_list.len() {
            if !self.free_list[i].is_empty() {
                for j in (class + 1..i + 1).rev() {
                    // split upper memory
                    if let Some(block) = self.free_list[j].pop() {
                        unsafe {
                            self.free_list[j - 1]
                                .push((block as usize + (1 << (j - 1))) as *mut usize);
                            self.free_list[j - 1].push(block);
                        }
                    } else {
                        return Err(());
                    }
                }

                let result = NonNull::new(
                    self.free_list[class]
                        .pop()
                        .expect("should have free memory") as *mut u8,
                );

                if let Some(result) = result {
                    self.allocated += size;
                    return Ok(result);
                } else {
                    return Err(());
                }
            }
        }

        Err(())
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = max(
            layout.size().next_power_of_two(),
            max(layout.align(), size_of::<usize>()),
        );

        let class = size.trailing_zeros() as usize;

        let mut current_class = class;
        let mut current_ptr = ptr.as_ptr() as usize;

        unsafe {
            self.free_list[class].push(current_ptr as *mut usize);
        }

        while current_class < self.free_list.len() {
            let mut merge_flag = false;
            let buddy = current_ptr ^ (1 << current_class);
            for block in self.free_list[current_class].iter_mut() {
                if block.value() as usize == buddy {
                    block.pop_current();
                    merge_flag = true;
                    break;
                }
            }

            if merge_flag {
                self.free_list[current_class].pop();
                current_ptr = min(current_ptr, buddy);
                current_class += 1;
                unsafe {
                    self.free_list[current_class].push(current_ptr as *mut usize);
                }
            } else {
                break;
            }
        }

        self.allocated -= size;
    }
}

impl<const ORDER: usize> fmt::Debug for Heap<ORDER> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Heap")
            .field("allocated", &self.allocated)
            .field("total", &self.total)
            .finish()
    }
}
