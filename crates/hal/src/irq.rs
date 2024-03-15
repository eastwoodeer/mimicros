static mut GLOBAL_HANDLER: usize = 0;

pub fn dispatch_irq_common(irq_num: usize) {
    let current_time = crate::time::current_time();
    trace!("[{}] handle irq... {}", current_time.as_nanos(), irq_num);

    crate::platform::timer::set_timer(current_time.as_nanos() as u64 + 1000000000);

    // task::on_timer_tick();
    let handler: fn() = unsafe { core::mem::transmute(GLOBAL_HANDLER) };
    handler();
}

pub fn register_irq_common(_irq_num: usize, handler: fn()) {
    unsafe { GLOBAL_HANDLER = handler as usize }
}
