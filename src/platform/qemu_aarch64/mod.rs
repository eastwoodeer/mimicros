pub mod entry;
pub mod mem;

pub mod timer {
    pub use crate::platform::aarch64::generic_timer::*;
}

pub mod gic {
    pub use crate::platform::aarch64::gic::*;
}

pub fn platform_init() {
    gic::init_primary();
    timer::init();
}
