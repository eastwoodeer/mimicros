use memory_addr::PhysAddr;
use page_table_entry::{aarch64::PTE, MemoryAttribute};

pub unsafe fn init_boot_page_table(
    boot_pgtable_l0: *mut [PTE; 512],
    boot_pgtable_l1: *mut [PTE; 512],
) {
    let pgtable_l0 = boot_pgtable_l0.as_mut().unwrap();
    let pgtable_l1 = boot_pgtable_l1.as_mut().unwrap();

    pgtable_l0[0] = PTE::new_table(PhysAddr::from(pgtable_l1.as_ptr() as usize));
    // 0 ~ 0x4000_0000 1G block device memory
    pgtable_l1[0] = PTE::new_page(
        PhysAddr::from(0),
        MemoryAttribute::READ | MemoryAttribute::WRITE | MemoryAttribute::DEVICE,
        true,
    );

    // 0x4000_0000 ~ 0x8000_0000 1G block kernel normal memory
    pgtable_l1[1] = PTE::new_page(
        PhysAddr::from(0x4000_0000),
        MemoryAttribute::READ | MemoryAttribute::WRITE | MemoryAttribute::EXECUTE,
        true,
    );
}
