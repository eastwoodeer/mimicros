static mut GLOBAL_HANDLER: usize = 0;

static mut NEXT_DEADLINE: u64 = 0;

// FIXME: config this value
const TICKS_PER_SEC: u64 = 10 ;
const NANOS_PER_SEC: u64 = 1_000_000_000;
const PERIODIC_INTERVAL_NANOS: u64 = NANOS_PER_SEC / TICKS_PER_SEC;

pub fn dispatch_irq_common(irq_num: usize) {
    let current_time = crate::time::current_time();
    // trace!("[{}] handle irq... {}", current_time.as_nanos(), irq_num);

    let mut deadline = unsafe { NEXT_DEADLINE };
    let now = crate::time::current_time_nanos();
    if now >= deadline {
        deadline = now + PERIODIC_INTERVAL_NANOS;
    }

    unsafe { NEXT_DEADLINE = deadline + PERIODIC_INTERVAL_NANOS };

    crate::platform::timer::set_timer(deadline);
    // task::on_timer_tick();
    let handler: fn() = unsafe { core::mem::transmute(GLOBAL_HANDLER) };
    handler();
}

pub fn register_irq_common(_irq_num: usize, handler: fn()) {
    unsafe { GLOBAL_HANDLER = handler as usize }
}
