#![cfg_attr(not(test), no_std)]

pub trait Preemptable {
    fn enable_preempt();
    fn disable_preempt();
}

pub trait IrqProtected {
    type State: Copy + Clone;

    fn irq_save() -> Self::State;
    fn irq_restore(state: Self::State);
}

pub struct NoOps;

impl IrqProtected for NoOps {
    type State = ();

    fn irq_save() -> Self::State {}
    fn irq_restore(_state: Self::State) {}
}

impl Preemptable for NoOps {
    fn enable_preempt() {}
    fn disable_preempt() {}
}

impl NoOps {
    pub const fn new() -> Self {
        Self
    }
}

extern "Rust" {
    fn __preempt_guard_enable_preempt();
    fn __preempt_guard_disable_preempt();
    fn local_irq_save() -> usize;
    fn local_irq_restore(flags: usize);
}

pub struct IrqSaveRestore;

impl IrqProtected for IrqSaveRestore {
    type State = usize;

    fn irq_save() -> Self::State {
        unsafe { local_irq_save() }
    }

    fn irq_restore(state: Self::State) {
        unsafe {
            local_irq_restore(state);
        }
    }
}

pub struct PreemptGuard;

impl Preemptable for PreemptGuard {
    fn enable_preempt() {
        unsafe { __preempt_guard_enable_preempt() }
    }

    fn disable_preempt() {
        unsafe { __preempt_guard_disable_preempt() }
    }
}

impl PreemptGuard {
    pub fn new() -> Self {
        Self::enable_preempt();
        Self
    }
}

impl Drop for PreemptGuard {
    fn drop(&mut self) {
        Self::disable_preempt();
    }
}
