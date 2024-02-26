pub fn dispatch_irq_common(irq_num: usize) {
    let current_time = crate::platform::time::current_time();
    trace!("[{}] handle irq... {}", current_time.as_nanos(), irq_num);

    crate::platform::timer::set_timer(current_time.as_nanos() as u64 + 1000000000);

    task::on_timer_tick();
}
