use aarch64_cpu::registers::TTBR1_EL1;
use tock_registers::interfaces::{Readable, Writeable};

use memory_addr::{PhysAddr, VirtAddr};

pub mod exception;


fn flush_tlb(vaddr: Option<VirtAddr>) {
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
