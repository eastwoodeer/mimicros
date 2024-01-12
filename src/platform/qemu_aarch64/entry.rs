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

pub extern "C" fn rust_start_main(cpuid: usize) {
    crate::mem::clear_bss();
    crate::arch::aarch64::exception::exception_init(exception_vector_base as usize);
    logger::init();
    info!("{}", LOGO);
    info!("boot cpuid: {}", cpuid);
    crate::mem::init_allocator();

    error!("panic here, it's ok");
    panic!("ends here");
}
