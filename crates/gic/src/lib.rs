#![no_std]
#![feature(const_nonnull_new)]
#![feature(const_option)]

pub mod gic_v2;

use core::ops::Range;

pub const SGI_RANGE: Range<usize> = 0..16;

pub const PPI_RANGE: Range<usize> = 16..32;

pub const SPI_RANGE: Range<usize> = 32..1020;

pub const MAX_IRQS: usize = 1024;

pub enum TriggerMode {
    Edge = 0,
    Level = 1,
}

pub enum InterruptType {
    // Software-Generated Interrupt
    SGI,
    // Private Peripheral Interrupt
    PPI,
    // Shared Peripheral Interrupt
    SPI,
}
