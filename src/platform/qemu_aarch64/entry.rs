use crate::println;

extern "C" {
    fn exception_vector_base();
}

pub extern "C" fn rust_start_main(cpuid: usize) {
    println!("cpuid: {}", cpuid);

    crate::mem::clear_bss();
    crate::arch::aarch64::exception::exception_init(exception_vector_base as usize);

    unsafe {
        core::ptr::write_volatile(0xD0000000 as *mut u8, 1 as u8);
    }

    panic!("ends here");
}
