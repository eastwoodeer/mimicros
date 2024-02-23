pub mod entry;
pub mod mem;

pub mod timer {
    pub use crate::platform::aarch64::generic_timer::{
        current_ticks, init, init_early, nanos_to_ticks, set_timer, ticks_to_nanos,
    };
}

pub mod irq {
    pub use crate::platform::aarch64::gic::{eoi, iar, init_primary, set_enable};
}

pub fn platform_init() {
    irq::init_primary();
    timer::init();
}
