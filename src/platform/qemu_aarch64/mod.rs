pub mod entry;
pub mod mem;

pub mod timer {
    pub use crate::platform::aarch64::generic_timer::{
        current_ticks, init, init_early, set_timer, ticks_to_nanos,
    };
}

pub mod irq {
    pub use crate::platform::aarch64::gic::{dispatch_irq, init_primary, set_enable};
}

pub fn platform_init() {
    irq::init_primary();
    timer::init();
}
