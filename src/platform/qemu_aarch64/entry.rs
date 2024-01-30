use memory_addr::{PhysAddr, VirtAddr};
use page_table::bits64::PageTable64;
use page_table::{PageSize, PagingError};
use page_table_entry::MemoryAttribute;

extern "C" {
    fn exception_vector_base();
}

const LOGO: &str = r#"
 __  __  _             _        ____    ___   ____
|  \/  |(_) _ __ ___  (_)  ___ |  _ \  / _ \ / ___|
| |\/| || || '_ ` _ \ | | / __|| |_) || | | |\___ \
| |  | || || | | | | || || (__ |  _ < | |_| | ___) |
|_|  |_||_||_| |_| |_||_| \___||_| \_\ \___/ |____/
"#;

fn remap_kernel_memory() -> Result<(), PagingError> {
    static mut KERNEL_PAGE_TABLE: PageTable64 = PageTable64::new();

    unsafe {
        KERNEL_PAGE_TABLE.memmap(
            VirtAddr::from(0 + 0xFFFF0000_00000000),
            PhysAddr::from(0),
            1 * 1024 * 1024 * 1024,
            MemoryAttribute::READ | MemoryAttribute::WRITE | MemoryAttribute::DEVICE,
        )?;
        KERNEL_PAGE_TABLE.memmap(
            VirtAddr::from(0x40000000 + 0xFFFF0000_00000000),
            PhysAddr::from(0x40000000),
            1 * 1024 * 1024 * 1024,
            MemoryAttribute::READ | MemoryAttribute::WRITE | MemoryAttribute::EXECUTE,
        )?;
    }

    Ok(())
}

pub extern "C" fn rust_start_main(cpuid: usize) {
    crate::mem::clear_bss();
    crate::arch::aarch64::exception::exception_init(exception_vector_base as usize);
    logger::init();
    info!("{}", LOGO);
    info!("boot cpuid: {}", cpuid);
    crate::mem::init_allocator();
    let mut pgt = page_table::bits64::PageTable64::new();
    // pgt.map(
    //     VirtAddr::from(0x100000000),
    //     PhysAddr::from(0x50000000),
    //     PageSize::Size4K,
    //     MemoryAttribute::READ | MemoryAttribute::WRITE,
    // ).expect("should be ok...");

    // unsafe {
    // 	core::ptr::write_volatile(0x100000000 as *mut u8, b'A');
    // }

    error!("panic here, it's ok");
    panic!("ends here");
}
