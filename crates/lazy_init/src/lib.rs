#![no_std]

use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::ops::{Deref, DerefMut};
use core::sync::atomic::{AtomicBool, Ordering};

pub struct LazyInit<T> {
    initialized: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
}

unsafe impl<T: Sync> Sync for LazyInit<T> {}

impl<T> LazyInit<T> {
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    pub fn init_by(&self, v: T) {
        assert!(!self.is_initialized());
        unsafe {
            (*self.value.get()).as_mut_ptr().write(v);
        }
        self.initialized.store(true, Ordering::Release);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized.load(Ordering::Acquire)
    }

    pub fn get(&self) -> &T {
        self.check_init();
        unsafe { &*(*self.value.get()).as_ptr() }
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.check_init();
        unsafe { &mut *(*self.value.get()).as_mut_ptr() }
    }

    pub fn check_init(&self) {
        if !self.is_initialized() {
            panic!("Use uninitialized LazyInit.");
        }
    }
}

impl<T> Deref for LazyInit<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for LazyInit<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T> Drop for LazyInit<T> {
    fn drop(&mut self) {
        if self.is_initialized() {
            unsafe {
                (*self.value.get()).as_mut_ptr().drop_in_place();
            }
        }
    }
}
