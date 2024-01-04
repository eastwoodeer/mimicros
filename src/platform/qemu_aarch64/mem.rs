pub unsafe fn init_boot_page_table(
    boot_pgtable_l0: &mut [PTE; 512],
    boot_pgtable_l1: &mut [PTE; 512],
) {
    BOOT_PGTABLE_L0[0] = PTE::new_table(PhysAddr::from(BOOT_PGTABLE_L1.as_ptr() as usize));
    // 0 ~ 0x4000_0000 1G block device memory
    BOOT_PGTABLE_L1[0] = PTE::new_page(
        PhysAddr::from(0),
        MemoryAttr::READ | MemoryAttr::WRITE | MemoryAttr::DEVICE,
        true,
    );

    // 0x4000_0000 ~ 0x8000_0000 1G block kernel normal memory
    BOOT_PGTABLE_L1[1] = PTE::new_page(
        PhysAddr::from(0x4000_0000),
        MemoryAttr::READ | MemoryAttr::WRITE | MemoryAttr::EXECUTE,
        true,
    );
}
