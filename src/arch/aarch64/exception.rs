use aarch64_cpu::registers::{VBAR_EL1, FAR_EL1};
use core::arch::global_asm;
use tock_registers::interfaces::{Writeable, Readable};

global_asm!(include_str!("exception.s"));

#[no_mangle]
fn invalid_exception(tf: u64, kind: u64, source: u64) {
    panic!("Invalid exception {} from {}, tr: {:#x}, far: {:#x}", kind, source, tf, FAR_EL1.get());
}

pub fn exception_init(vbar_el1: usize) {
    VBAR_EL1.set(vbar_el1 as _);
}
