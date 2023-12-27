#[no_mangle]
pub static BOOT_CORE_ID: u64 = 0;

#[no_mangle]
pub static CPACR_ZEN_BIT: u64 = 16;

#[no_mangle]
pub static CPACR_FPEN_BIT: u64 = 20;

#[no_mangle]
pub static ZEN_NO_TRAP: u64 = 3;

#[no_mangle]
pub static FPEN_NO_TRAP: u64 = 3;

#[no_mangle]
pub static boot_stack: u64 = 0x120000;
