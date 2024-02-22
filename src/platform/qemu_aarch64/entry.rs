use lazy_init::LazyInit;
use memory_addr::{PhysAddr, VirtAddr};
use page_table::bits64::PageTable64;
use page_table::{PageSize, PagingError};
use page_table_entry::MemoryAttribute;
use ratio::Ratio;

use crate::platform::{self, timer, gic};

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
    static KERNEL_PAGE_TABLE: LazyInit<PageTable64> = LazyInit::new();

    let mut page_table = PageTable64::new();

    page_table.memmap(
        VirtAddr::from(0 + 0xFFFF0000_00000000),
        PhysAddr::from(0),
        1 * 1024 * 1024 * 1024,
        MemoryAttribute::READ | MemoryAttribute::WRITE | MemoryAttribute::DEVICE,
    )?;
    page_table.memmap(
        VirtAddr::from(0x40000000 + 0xFFFF0000_00000000),
        PhysAddr::from(0x40000000),
        1 * 1024 * 1024 * 1024,
        MemoryAttribute::READ | MemoryAttribute::WRITE | MemoryAttribute::EXECUTE,
    )?;

    KERNEL_PAGE_TABLE.init_by(page_table);

    crate::arch::write_page_table_root(KERNEL_PAGE_TABLE.root_addr());

    Ok(())
}

fn init_interrupt() {
    let current_time = platform::time::current_time_nanos();

    timer::set_timer(current_time + 10000000);
    gic::set_enable(30, true);

    crate::arch::enable_irqs();
}

pub extern "C" fn rust_start_main(cpuid: usize) {
    crate::mem::clear_bss();
    crate::arch::exception::exception_init(exception_vector_base as usize);

    timer::init_early();
    logger::init();

    info!("{}", LOGO);

    info!("boot cpuid: {}", cpuid);

    crate::mem::init_allocator();
    remap_kernel_memory().expect("remap kernel memory failed.");
    info!("kenel memory initialized.....");

    // let r = Ratio::new(99999999, 3);
    // debug!("{:?}", r);

    platform::platform_init();

    init_interrupt();


    error!("panic here, it's ok");
    panic!("ends here");
}
