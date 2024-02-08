#![cfg_attr(not(test), no_std)]

use core::cell::UnsafeCell;
use core::marker::{PhantomData, Send, Sync};
use core::ops::{Deref, DerefMut, Drop};
use core::sync::atomic::{
    AtomicBool,
    Ordering::{Acquire, Relaxed, Release},
};

// NoPreempt, Optional(IrqSave)

pub trait PreemptGuard {
    fn enable_preempt();
    fn disable_preempt();
}

pub trait IrqGuard {
    type State: Copy + Clone;

    fn irq_save() -> Self::State;
    fn irq_restore(state: Self::State);
}

pub struct NoOps;

impl IrqGuard for NoOps {
    type State = ();

    fn irq_save() -> Self::State {}
    fn irq_restore(_state: Self::State) {}
}

impl PreemptGuard for NoOps {
    fn enable_preempt() {}
    fn disable_preempt() {}
}

impl NoOps {
    pub const fn new() -> Self {
        Self
    }
}

pub type SpinLock<T> = SpinLockRaw<T, NoOps, NoOps>;

pub struct SpinLockRaw<T, G: IrqGuard, P: PreemptGuard> {
    _phantom: PhantomData<(G, P)>,
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
unsafe impl<T, G: IrqGuard, P: PreemptGuard> Sync for SpinLockRaw<T, G, P> where T: Send {}

pub struct SpinGuard<'a, T, G: IrqGuard, P: PreemptGuard> {
    lock: &'a SpinLockRaw<T, G, P>,
    irq_state: G::State,
}

impl<T, G: IrqGuard, P: PreemptGuard> SpinLockRaw<T, G, P> {
    pub const fn new(v: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(v),
            _phantom: PhantomData,
        }
    }

    pub fn lock(&self) -> SpinGuard<T, G, P> {
        P::disable_preempt();
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

impl<T, G: IrqGuard, P: PreemptGuard> Deref for SpinGuard<'_, T, G, P> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T, G: IrqGuard, P: PreemptGuard> DerefMut for SpinGuard<'_, T, G, P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T, G: IrqGuard, P: PreemptGuard> Drop for SpinGuard<'_, T, G, P> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release);
        G::irq_restore(self.irq_state);
        P::enable_preempt();
    }
}
