#![cfg_attr(not(test), no_std)]

#[macro_use]
extern crate percpu_macro;

pub use percpu_macro::define_per_cpu;

#[define_per_cpu]
pub static EXAMPLE_PERCPU_DATA: u32 = 0;
