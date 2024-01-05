use aarch64_cpu::registers::VBAR_EL1;
use core::arch::global_asm;
use tock_registers::interfaces::Writeable;

global_asm!(include_str!("exception.s"));

#[no_mangle]
fn invalid_exception(tf: u64, kind: u64, source: u64) {
    panic!("Invalid exception {} from {}, tr: {}", kind, source, tf);
}

extern "C" {
    fn exception_vector_base();
}

pub fn exception_init() {
    VBAR_EL1.set(exception_vector_base as usize as _);
}
