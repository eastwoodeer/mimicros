#![cfg_attr(not(test), no_std)]

use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};
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

pub struct IrqSave;

impl IrqGuard for IrqSave {
    type State = usize;

    fn irq_save() -> Self::State {
        hal::arch::local_irq_save()
    }

    fn irq_restore(state: Self::State) {
        hal::arch::local_irq_restore(state)
    }
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

extern "Rust" {
    fn __PreemptGuard_enable_preempt();
    fn __PreemptGuard_disable_preempt();
}

pub struct TaskGuard;

impl PreemptGuard for TaskGuard {
    fn enable_preempt() {
        // task::enable_preempt()
        unsafe { __PreemptGuard_enable_preempt() }
    }

    fn disable_preempt() {
        // task::disable_preempt()
        unsafe { __PreemptGuard_disable_preempt() }
    }
}

pub type SpinLock<T> = SpinLockPrototype<T, NoOps, NoOps>;
pub type SpinNoIrq<T> = SpinLockPrototype<T, IrqSave, TaskGuard>;

pub struct SpinLockPrototype<T, G: IrqGuard, P: PreemptGuard> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
    _phantom: PhantomData<(G, P)>,
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
unsafe impl<T, G: IrqGuard, P: PreemptGuard> Sync for SpinLockPrototype<T, G, P> where T: Send {}

pub struct SpinGuard<'a, T, G: IrqGuard, P: PreemptGuard> {
    lock: &'a SpinLockPrototype<T, G, P>,
    irq_state: G::State,
}

impl<T, G: IrqGuard, P: PreemptGuard> SpinLockPrototype<T, G, P> {
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

    pub unsafe fn force_unlock(&self) {
        self.locked.store(false, Release);
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
