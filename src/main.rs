#![no_std]
#![no_main]
#![feature(asm_const)]

mod arch;
mod console;
mod cpu;
mod platform;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
