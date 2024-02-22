use core::time::Duration;

pub use crate::platform::timer::{current_ticks, nanos_to_ticks, ticks_to_nanos};

pub fn current_time_nanos() -> u64 {
    ticks_to_nanos(current_ticks())
}

pub fn current_time() -> Duration {
    Duration::from_nanos(current_time_nanos())
}
