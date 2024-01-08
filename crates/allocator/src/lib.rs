#![cfg_attr(not(test), no_std)]

mod linked_list;
mod test;

pub struct Heap<const ORDER: usize> {
    free_list: [linked_list::LinkedList; ORDER],
    used: usize,
    allocated: usize,
    total: usize,
}
