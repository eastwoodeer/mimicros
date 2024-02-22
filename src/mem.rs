use memory_addr::{PhysAddr, VirtAddr};

extern "C" {
    fn __bss_start();
    fn __bss_end();
}

pub fn clear_bss() {
    // crate::println!("bss start 0x{:x}, end: 0x{:x}", __bss_start as usize, __bss_end as usize);
    unsafe {
        core::slice::from_raw_parts_mut(
            __bss_start as *mut u8,
            __bss_end as usize - __bss_start as usize,
        )
        .fill(0);
    }
}

pub fn init_allocator() {
    allocator::global_init(__bss_end as usize, 900 * 1024 * 1024);
}

#[inline]
pub const fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr::from(paddr.as_usize() + 0xFFFF_0000_0000_0000)
}
