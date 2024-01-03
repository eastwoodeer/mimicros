#![no_std]
#![no_main]
#![feature(asm_const)]

use aarch64_cpu::registers::*;
use core::panic::PanicInfo;

mod platform;
mod console;
mod cpu;
mod exception;
mod pagetable;

extern "C" {
    fn exception_vector_base();
}

fn kernel_init() -> ! {
    VBAR_EL1.set(exception_vector_base as usize as _);

    panic!("bad end.");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
