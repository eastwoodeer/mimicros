#![feature(naked_functions)]
#![feature(asm_const)]
#![cfg_attr(not(test), no_std)]

pub mod arch;
pub mod irq;
pub mod mem;
pub mod platform;

#[macro_use]
pub mod console;

#[macro_use]
extern crate log;
