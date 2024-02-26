#![cfg_attr(not(test), no_std)]
#![no_main]
#![feature(asm_const)]

#[macro_use]
extern crate log;

mod arch;
mod console;
mod cpu;
mod irq;
mod mem;
mod platform;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
