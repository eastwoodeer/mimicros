#[inline]
pub fn current_task_ptr<T>() -> *const T {
    // On aarch64, use sp_el0 as current task pointer.
    use tock_registers::interfaces::Readable;
    aarch64_cpu::registers::SP_EL0.get() as _
}

#[inline]
pub fn set_current_task_ptr<T>(ptr: *const T) {
    use tock_registers::interfaces::Writeable;
    aarch64_cpu::registers::SP_EL0.set(ptr as u64)
}
