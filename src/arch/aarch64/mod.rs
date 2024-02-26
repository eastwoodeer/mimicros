use core::arch::asm;

pub mod exception;

use aarch64_cpu::registers::TTBR1_EL1;
use tock_registers::interfaces::Writeable;

use memory_addr::{PhysAddr, VirtAddr};

/// Flushes the TLB.
///
/// If `vaddr` is [`None`], flushes the entire TLB. Otherwise, flushes the TLB
/// entry that maps the given virtual address.
#[inline]
pub fn flush_tlb(vaddr: Option<VirtAddr>) {
    unsafe {
        if let Some(vaddr) = vaddr {
            asm!("tlbi vaae1is, {}; dsb sy; isb", in(reg) vaddr.as_usize())
        } else {
            // flush the entire TLB
            asm!("tlbi vmalle1; dsb sy; isb")
        }
    }
}

pub fn write_page_table_root(root_addr: PhysAddr) {
    TTBR1_EL1.set(root_addr.as_usize() as _);
    flush_tlb(None);
}
