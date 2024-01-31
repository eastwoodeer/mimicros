use aarch64_cpu::registers::CNTFRQ_EL0;
use tock_registers::interfaces::{Readable, Writeable};

pub fn init_early() {
	let freq = CNTFRQ_EL0.get();

	debug!("freq: {:?}", freq);
}
