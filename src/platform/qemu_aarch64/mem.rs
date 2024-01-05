use page_table_entry::{MemoryAttr, aarch64::PTE};
use memory_addr::PhysAddr;

pub unsafe fn init_boot_page_table(
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

    // 0x4000_0000 ~ 0x8000_0000 1G block kernel normal memory
    boot_pgtable_l1[1] = PTE::new_page(
        PhysAddr::from(0x4000_0000),
        MemoryAttr::READ | MemoryAttr::WRITE | MemoryAttr::EXECUTE,
        true,
    );
}
