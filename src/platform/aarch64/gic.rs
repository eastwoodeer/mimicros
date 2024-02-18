use gic::gic_v2::{GicCpuInterface, GicDistributor};
use crate::mem::phys_to_virt;
use memory_addr::PhysAddr;
use spinlock::SpinNoIrq;

pub const MAX_IRQ_COUNT: usize = 1024;

const GICD_BASE: PhysAddr = PhysAddr::from(0x0800_0000);
const GICC_BASE: PhysAddr = PhysAddr::from(0x0801_0000);

static GICD: SpinNoIrq<GicDistributor> = SpinNoIrq::new(GicDistributor::new(phys_to_virt(GICD_BASE).as_mut_ptr()));
static GICC: GicCpuInterface = GicCpuInterface::new(phys_to_virt(GICC_BASE).as_mut_ptr());

pub fn set_enable(irq_num: usize, enable: bool) {}


pub fn init_primary() {
	info!("init GICv2.");
	GICD.lock().init();
	GICC.init();
}
