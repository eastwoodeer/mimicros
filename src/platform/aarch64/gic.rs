use crate::mem::phys_to_virt;
use gic::gic_v2::{GicCpuInterface, GicDistributor};
use memory_addr::PhysAddr;
use spinlock::SpinNoIrq;

// pub const MAX_IRQ_COUNT: usize = 1024;
// pub const NON_SECURE_PHYSICAL_TIMER_INTID: usize = 30;

const GICD_BASE: PhysAddr = PhysAddr::from(0x0800_0000);
const GICC_BASE: PhysAddr = PhysAddr::from(0x0801_0000);

static GICD: SpinNoIrq<GicDistributor> =
    SpinNoIrq::new(GicDistributor::new(phys_to_virt(GICD_BASE).as_mut_ptr()));
static GICC: GicCpuInterface = GicCpuInterface::new(phys_to_virt(GICC_BASE).as_mut_ptr());

pub fn set_enable(irq_num: usize, enable: bool) {
    GICD.lock().set_enable(irq_num, enable)
}

pub fn init_primary() {
    info!("init GICv2.");
    GICD.lock().init();
    GICC.init();
}

pub fn iar() -> u32 {
    GICC.iar()
}

pub fn eoi(iar: u32) {
    GICC.eoi(iar)
}

pub fn dispatch_irq(irq_num: usize) {
    GICC.handle_irq(|irq_num| crate::irq::dispatch_irq_common(irq_num as _));
}
