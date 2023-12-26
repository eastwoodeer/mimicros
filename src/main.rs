#![no_std]
#![no_main]
#![feature(asm_const)]

use core::panic::PanicInfo;

mod boot;
mod cpu;
mod console;

fn kernel_init() -> ! {
    println!("Hello world");

    panic!("bad end.");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unimplemented!()
}
