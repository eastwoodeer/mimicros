use crate::linked_list::LinkedList;

// use core::prelude::rust_2024::{test, derive};

fn test_linked_list() {
    let mut addr1: usize = 0;
    let mut addr2: usize = 0;
    let mut addr3: usize = 0;

    let mut list = LinkedList::new();
    unsafe {
        list.push(&mut addr1 as *mut usize);
        list.push(&mut addr2 as *mut usize);
        list.push(&mut addr3 as *mut usize);
    }
}
