#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod arch;
mod platform;
mod console;
mod cpu;

fn kernel_init() -> ! {
    crate::arch::aarch64::exception::exception_init();

    panic!("bad end.");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
