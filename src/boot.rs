use aarch64_cpu::{asm, asm::barrier, registers::*};
use tock_registers::interfaces::{ReadWriteable, Readable, Writeable};

use crate::pagetable::{MemoryAttr, PhysAddr, VirtAddr, PTE};

use core::arch::{asm, global_asm};

#[link_section = ".boot.stack"]
static mut BOOT_STACK: [u8; 40960] = [0; 40960];

#[link_section = ".data.stack"]
static mut BOOT_PGTABLE_L0: [PTE; 512] = [PTE::empty(); 512];

#[link_section = ".data.stack"]
static mut BOOT_PGTABLE_L1: [PTE; 512] = [PTE::empty(); 512];

global_asm!(
    include_str!("boot.s"),
    CONST_CORE_ID_MASK = const 0b11,
    init_boot_page_table = sym init_boot_page_table,
    init_mmu = sym init_mmu,
    switch_to_el1 = sym switch_to_el1,
    enable_fp = sym enable_fp,
    boot_stack = sym BOOT_STACK);

unsafe fn _init_boot_page_table(
    boot_pgtable_l0: &mut [PTE; 512],
    boot_pgtable_l1: &mut [PTE; 512],
) {
    boot_pgtable_l0[0] = PTE::new_table(PhysAddr::from(boot_pgtable_l1.as_ptr() as usize));
    // 0 ~ 0x4000_0000 1G block device memory
    boot_pgtable_l1[0] = PTE::new_page(
        PhysAddr::from(0),
        MemoryAttr::READ | MemoryAttr::WRITE | MemoryAttr::DEVICE,
        true,
    );

    // 0x4000_0000 ~ 0x8000_0000 1G block normal memory
    boot_pgtable_l1[1] = PTE::new_page(
        PhysAddr::from(0x4000_0000),
        MemoryAttr::READ | MemoryAttr::WRITE | MemoryAttr::EXECUTE,
        true,
    );
}

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

unsafe fn init_mmu() {
    // Device memory nGnRE
    let attr0 = MAIR_EL1::Attr0_Device::nonGathering_nonReordering_EarlyWriteAck;
    // Normal memory
    let attr1 = MAIR_EL1::Attr1_Normal_Inner::WriteBack_NonTransient_ReadWriteAlloc
        + MAIR_EL1::Attr1_Normal_Outer::WriteBack_NonTransient_ReadWriteAlloc;
    MAIR_EL1.write(attr0 + attr1);

    // Enable TTBR0 and TTBR1 walks, page size = 4K, vaddr size = 48 bits, paddr size = 40 bits.
    let tcr_flags0 = TCR_EL1::EPD0::EnableTTBR0Walks
        + TCR_EL1::TG0::KiB_4
        + TCR_EL1::SH0::Inner
        + TCR_EL1::ORGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN0::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T0SZ.val(16);
    let tcr_flags1 = TCR_EL1::EPD1::EnableTTBR1Walks
        + TCR_EL1::TG1::KiB_4
        + TCR_EL1::SH1::Inner
        + TCR_EL1::ORGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::IRGN1::WriteBack_ReadAlloc_WriteAlloc_Cacheable
        + TCR_EL1::T1SZ.val(16);
    TCR_EL1.write(TCR_EL1::IPS::Bits_48 + tcr_flags0 + tcr_flags1);
    barrier::isb(barrier::SY);

    // Set both TTBR0 and TTBR1
    let root_paddr = PhysAddr::from(BOOT_PGTABLE_L0.as_ptr() as usize).as_usize() as _;
    TTBR0_EL1.set(root_paddr);
    TTBR1_EL1.set(root_paddr);

    // Flush the entire TLB
    flush_tlb(None);

    // Enable the MMU and turn on I-cache and D-cache
    SCTLR_EL1.modify(SCTLR_EL1::M::Enable + SCTLR_EL1::C::Cacheable + SCTLR_EL1::I::Cacheable);
    barrier::isb(barrier::SY);
}

unsafe fn init_boot_page_table() {
    _init_boot_page_table(&mut BOOT_PGTABLE_L0, &mut BOOT_PGTABLE_L1);
}

fn switch_to_el1() {
    SPSel.write(SPSel::SP::ELx);
    SP_EL0.set(0);

    let current_el = CurrentEL.read(CurrentEL::EL);
    if current_el > 1 {
        if current_el == 3 {
            SCR_EL3.write(
                SCR_EL3::NS::NonSecure + SCR_EL3::HCE::HvcEnabled + SCR_EL3::RW::NextELIsAarch64,
            );
            SPSR_EL3.write(
                SPSR_EL3::M::EL1h
                    + SPSR_EL3::D::Masked
                    + SPSR_EL3::A::Masked
                    + SPSR_EL3::I::Masked
                    + SPSR_EL3::F::Masked,
            );
            ELR_EL3.set(LR.get());
        }

        CNTHCTL_EL2.modify(CNTHCTL_EL2::EL1PCEN::SET + CNTHCTL_EL2::EL1PCTEN::SET);
        CNTVOFF_EL2.set(0);
        HCR_EL2.write(HCR_EL2::RW::EL1IsAarch64);
        SPSR_EL2.write(
            SPSR_EL2::M::EL1h
                + SPSR_EL2::D::Masked
                + SPSR_EL2::A::Masked
                + SPSR_EL2::I::Masked
                + SPSR_EL2::F::Masked,
        );

        SP_EL1.set(SP.get());
        ELR_EL2.set(LR.get());
        asm::eret();
    }
}

fn enable_fp() {
    CPACR_EL1.write(CPACR_EL1::FPEN::TrapNothing);
    barrier::isb(barrier::SY);
}

#[no_mangle]
pub fn _start_rust() -> ! {
    crate::kernel_init()
}
