use core::arch::asm;

pub mod exception;

use aarch64_cpu::registers::TTBR1_EL1;
use tock_registers::interfaces::Writeable;

use memory_addr::{PhysAddr, VirtAddr};

#[inline]
pub fn enable_irqs() {
    unsafe { asm!("msr daifclr, 2") };
}

#[inline]
pub fn disable_irqs() {
    unsafe { asm!("msr daifset, 2") };
}

/// Flushes the TLB.
///
/// If `vaddr` is [`None`], flushes the entire TLB. Otherwise, flushes the TLB
/// entry that maps the given virtual address.
#[inline]
pub fn flush_tlb(vaddr: Option<VirtAddr>) {
    unsafe {
        if let Some(vaddr) = vaddr {
            core::arch::asm!("tlbi vaae1is, {}; dsb sy; isb", in(reg) vaddr.as_usize())
        } else {
            // flush the entire TLB
            core::arch::asm!("tlbi vmalle1; dsb sy; isb")
        }
    }
}

pub fn write_page_table_root(root_addr: PhysAddr) {
    TTBR1_EL1.set(root_addr.as_usize() as _);
    flush_tlb(None);
}
