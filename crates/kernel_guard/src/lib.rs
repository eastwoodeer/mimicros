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

pub struct IrqSaveRestore;

impl IrqProtected for IrqSaveRestore {
    type State = usize;

    fn irq_save() -> Self::State {
        hal::arch::local_irq_save()
    }

    fn irq_restore(state: Self::State) {
        hal::arch::local_irq_restore(state)
    }
}

extern "Rust" {
    fn __preempt_guard_enable_preempt();
    fn __preempt_guard_disable_preempt();
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
