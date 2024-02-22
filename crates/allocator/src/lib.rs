#![cfg_attr(not(test), no_std)]

pub mod buddy;
mod linked_list;

#[macro_use]
extern crate log;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;

use spinlock::SpinLock;

#[derive(Debug)]
pub enum AllocError {
    InvalidParam,
    NoMemory,
}

pub type AllocResult<T = ()> = Result<T, AllocError>;

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

    pub fn alloc_pages(&self, pages: usize) -> AllocResult<usize> {
        let layout =
            Layout::from_size_align(4096 * pages, 4096).map_err(|_| AllocError::InvalidParam)?;
        let ptr = self.inner.lock().alloc(layout)?;
        Ok(ptr.as_ptr() as usize)
    }

    pub fn dealloc_pages(&self, ptr: *mut u8, pages: usize) {
        let ptr = NonNull::new(ptr).expect("expect none NULL pointer.");
        let layout = Layout::from_size_align(4096 * pages, 4096).expect("unexpected layout");
        self.inner.lock().dealloc(ptr, layout);
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
    debug!(
        "init global allocator start at {:#x}, size {:#x}",
        start, size
    );
    GLOBAL_ALLOCATOR.init(start, size);
}

pub fn global_allocator() -> &'static GlobalAllocator {
    &GLOBAL_ALLOCATOR
}
