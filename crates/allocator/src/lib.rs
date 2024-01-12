#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate log;

pub mod buddy;
mod linked_list;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use spinlock::SpinLock;

pub struct GlobalAllocator {
    inner: SpinLock<buddy::BuddyAllocator>,
}

impl GlobalAllocator {
    pub const fn new() -> Self {
        Self {
            inner: SpinLock::new(buddy::BuddyAllocator::new()),
        }
    }

    pub fn init(&self, start: usize, size: usize) {
        self.inner.lock().add_memory(start, size);
    }
}

unsafe impl GlobalAlloc for GlobalAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if let Ok(ptr) = self.inner.lock().alloc(layout) {
            ptr.as_ptr()
        } else {
            panic!("bad alloc");
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.inner
            .lock()
            .dealloc(NonNull::new(ptr).expect("dealloc NULL pointer"), layout)
    }
}

#[global_allocator]
static GLOBAL_ALLOCATOR: GlobalAllocator = GlobalAllocator::new();

pub fn global_init(start: usize, size: usize) {
    info!("start at {:#x}, size {:#x}", start, size);
    GLOBAL_ALLOCATOR.init(start, size);
}
