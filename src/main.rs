#![no_std]
#![no_main]
#![feature(asm_const)]

mod arch;
mod console;
mod cpu;
mod platform;

use core::panic::PanicInfo;

fn kernel_init() -> ! {
    crate::arch::aarch64::exception::exception_init();

    panic!("bad end.");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
