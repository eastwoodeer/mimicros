use aarch64_cpu::registers::{ESR_EL1, FAR_EL1, VBAR_EL1};
use core::arch::global_asm;
use tock_registers::interfaces::{Readable, Writeable};

use kernel_guard::{PreemptGuard, Preemptable};

global_asm!(include_str!("exception.s"));

#[no_mangle]
fn invalid_exception(tf: u64, kind: u64, source: u64) {
    panic!(
        "Invalid exception {} from EL{}, tr: {:#x}, far: {:#x}, esr: {:#x}",
        kind,
        source,
        tf,
        FAR_EL1.get(),
        ESR_EL1.get()
    );
}

#[no_mangle]
fn handle_irq(_tf: u64) {
    // RUN_QUEUE would disable preempt, if we don't disable preempt here, reschedule would
    // run after RUN_QUEUE unlock.
    PreemptGuard::disable_preempt();
    crate::platform::irq::dispatch_irq();
    PreemptGuard::enable_preempt();
}

pub fn exception_init(vbar_el1: usize) {
    VBAR_EL1.set(vbar_el1 as _);
}
