#![no_std]

pub mod aarch64;

use core::fmt::Debug;

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct MemoryAttribute: usize {
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
