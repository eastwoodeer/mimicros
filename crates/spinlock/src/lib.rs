#![cfg_attr(not(test), no_std)]

use core::cell::UnsafeCell;
use core::marker::{PhantomData, Send, Sync};
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};

// NoPreempt, Optional(IrqSave)

pub trait KernelGuard {
    fn enable_preempt();
    fn disable_preempt();
}

pub trait IrqGuard {
    type State: Copy + Clone;

    fn irq_save() -> Self::State;
    fn irq_restore(state: Self::State);
}

pub struct SpinLock<T, G: IrqGuard> {
    _phantom: PhantomData<G>,
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
unsafe impl<T, G: IrqGuard> Sync for SpinLock<T, G> where T: Send {}

pub struct SpinGuard<'a, T, G: IrqGuard> {
    lock: &'a SpinLock<T, G>,
    irq_state: G::State,
}

impl<T, G: IrqGuard> SpinLock<T, G> {
    pub const fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(v),
        }
    }

    pub fn lock(&self) -> SpinGuard<T, G> {
        let irq_state = G::irq_save();
        while self
            .locked
            .compare_exchange_weak(false, true, Acquire, Relaxed)
            .is_err()
        {
            core::hint::spin_loop();
        }

        SpinGuard {
            lock: self,
            irq_state,
        }
    }
}

impl<T, G: IrqGuard> Deref for SpinGuard<'_, T, G> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T, G: IrqGuard> DerefMut for SpinGuard<'_, T, G> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T, G: IrqGuard> Drop for SpinGuard<'_, T, G> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
		G::irq_restore(self.irq_state);
    }
}
