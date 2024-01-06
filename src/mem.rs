extern "C" {
    fn __bss_start();
    fn __bss_end();
}

pub fn clear_bss() {
    crate::println!("bss start 0x{:x}, end: 0x{:x}", __bss_start as usize, __bss_end as usize);
    unsafe {
        core::slice::from_raw_parts_mut(
            __bss_start as *mut u8,
            __bss_end as usize - __bss_start as usize,
        ).fill(0);
    }
}
