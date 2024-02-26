use core::arch::asm;

#[inline]
pub fn enable_irqs() {
    unsafe { asm!("msr daifclr, 2") };
}

#[inline]
pub fn disable_irqs() {
    unsafe { asm!("msr daifset, 2") };
}

#[inline]
pub fn local_irq_save() -> usize {
    let flags: usize;
    unsafe { asm!("mrs {}, daif; msr daifset, 2", out(reg) flags) };
    return flags;
}

#[inline]
pub fn local_irq_restore(flags: usize) {
    unsafe { asm!("msr daif, {}", in(reg) flags) };
}
