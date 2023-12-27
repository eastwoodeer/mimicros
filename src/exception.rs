use core::arch::global_asm;

global_asm!(
    include_str!("exception.s")
);

#[no_mangle]
fn invalid_exception(tf: u64, kind: u64, source: u64) {
    panic!("Invalid exception {} from {}, tr: {}", kind, source, tf);
}
