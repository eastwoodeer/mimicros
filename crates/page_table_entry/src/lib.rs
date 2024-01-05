#![no_std]

use core::fmt::Debug;

pub mod aarch64;

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct MemoryAttr: usize {
        /// the memory is readable.
        const READ    = 1 << 0;
        /// the memory is writable.
        const WRITE   = 1 << 1;
        /// the memory is executable.
        const EXECUTE = 1 << 2;
        /// the memory is user accessible.
        const USER    = 1 << 3;
        /// the memory is device memory.
        const DEVICE  = 1 << 4;
    }
}
