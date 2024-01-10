#![cfg_attr(not(test), no_std)]

use core::cell::UnsafeCell;
use core::marker::{Send, Sync};
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

// UnsafeCell does not implement Sync, which means that our type is no longer
// shareable between threads.
// To fix that, we have to promise to compiler that it is actually safe to share between threads
// However, since the lock can be used to send values of type T from one thread to another, we
// must limit this promise to types that are safe to send between threads.
// So we implement Sync for SpinLock<T> for all T that implement Send.
//
// Note: We don't need T is Sync, because SpinLock<T> will only allow one thread at a time to
// access the T it protects.
unsafe impl<T> Sync for SpinLock<T> where T: Send {}

pub struct SpinGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    pub fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self) -> SpinGuard<T> {
        while self
            .locked
            .compare_exchange_weak(false, true, Acquire, Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        SpinGuard { lock: self }
    }
}

impl<T> Deref for SpinGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for SpinGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for SpinGuard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
    }
}
