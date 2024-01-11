use crate::println;

extern "C" {
    fn exception_vector_base();
}

pub extern "C" fn rust_start_main(cpuid: usize) {
    println!("cpuid: {}", cpuid);

    crate::mem::clear_bss();
    crate::arch::aarch64::exception::exception_init(exception_vector_base as usize);
    crate::mem::init_allocator();


    panic!("ends here");
}
