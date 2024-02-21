mod boot;

pub mod generic_timer;
pub mod gic;

pub mod timer {
    pub use crate::platform::aarch64::generic_timer::*;
}
