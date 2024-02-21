use aarch64_cpu::registers::{CNTFRQ_EL0, CNTPCT_EL0, CNTP_CTL_EL0, CNTP_TVAL_EL0};
use ratio::Ratio;
use tock_registers::interfaces::{Readable, Writeable};

static mut CNTPCT_TO_NANOS_RATIO: Ratio = Ratio::new(0, 0);
static mut NANOS_TO_CNTPCT_RATIO: Ratio = Ratio::new(0, 0);

#[inline]
pub fn current_ticks() -> u64 {
    CNTPCT_EL0.get()
}

#[inline]
pub fn ticks_to_nanos(ticks: u64) -> u64 {
    unsafe { CNTPCT_TO_NANOS_RATIO.multiply(ticks) }
}

#[inline]
pub fn nanos_to_ticks(nanos: u64) -> u64 {
    unsafe { NANOS_TO_CNTPCT_RATIO.multiply(nanos) }
}

pub fn current_time_nanos() -> u64 {
    ticks_to_nanos(current_ticks())
}

pub fn set_timer(deadline_ns: u64) {
    let current_ticks = current_ticks();
    let deadline_ticks = nanos_to_ticks(deadline_ns);

    if current_ticks < deadline_ticks {
        let interval = deadline_ticks - current_ticks;
        CNTP_TVAL_EL0.set(interval);

        debug!("interval: {}, {}", interval, CNTP_TVAL_EL0.get());
    } else {
        CNTP_TVAL_EL0.set(0);
    }
}

pub fn init_early() {
    let freq = CNTFRQ_EL0.get();

    unsafe {
        CNTPCT_TO_NANOS_RATIO = Ratio::new(1_000_000_000, freq as u32);
        NANOS_TO_CNTPCT_RATIO = Ratio::new(freq as u32, 1_000_000_000);
    }

    unsafe {
        debug!(
            "freq: {:?}, CNTPCT_TO_NANOS_RATIO: {:?}, NANOS_TO_CNTPCT_RATIO: {:?}",
            freq, CNTPCT_TO_NANOS_RATIO, NANOS_TO_CNTPCT_RATIO
        );
    }
}

pub fn init() {
    CNTP_CTL_EL0.write(CNTP_CTL_EL0::ENABLE::SET);
	CNTP_TVAL_EL0.set(0);
}

pub fn init_generic_timer() {}
