use core::ops::RangeBounds;
use core::ptr::NonNull;

use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_structs;
use tock_registers::registers::{ReadOnly, ReadWrite, WriteOnly};

use crate::{TriggerMode, MAX_IRQS, SPI_RANGE};

register_structs! {
    #[allow(non_snake_case)]
    GicDistributorRegs {
        /// Distributor Control Register.
        (0x0000 => CTLR: ReadWrite<u32>),
        /// Interrupt Controller Type Register.
        (0x0004 => TYPER: ReadOnly<u32>),
        /// Distributor Implementer Identification Register.
        (0x0008 => IIDR: ReadOnly<u32>),
        (0x000c => _reserved_0),
        /// Interrupt Group Registers.
        (0x0080 => IGROUPR: [ReadWrite<u32>; 0x20]),
        /// Interrupt Set-Enable Registers.
        (0x0100 => ISENABLER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Clear-Enable Registers.
        (0x0180 => ICENABLER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Set-Pending Registers.
        (0x0200 => ISPENDR: [ReadWrite<u32>; 0x20]),
        /// Interrupt Clear-Pending Registers.
        (0x0280 => ICPENDR: [ReadWrite<u32>; 0x20]),
        /// Interrupt Set-Active Registers.
        (0x0300 => ISACTIVER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Clear-Active Registers.
        (0x0380 => ICACTIVER: [ReadWrite<u32>; 0x20]),
        /// Interrupt Priority Registers.
        (0x0400 => IPRIORITYR: [ReadWrite<u32>; 0x100]),
        /// Interrupt Processor Targets Registers.
        (0x0800 => ITARGETSR: [ReadWrite<u32>; 0x100]),
        /// Interrupt Configuration Registers.
        (0x0c00 => ICFGR: [ReadWrite<u32>; 0x40]),
        (0x0d00 => _reserved_1),
        /// Software Generated Interrupt Register.
        (0x0f00 => SGIR: WriteOnly<u32>),
        (0x0f04 => @END),
    }
}

register_structs! {
    /// GIC CPU Interface registers.
    #[allow(non_snake_case)]
    GicCpuInterfaceRegs {
        /// CPU Interface Control Register.
        (0x0000 => CTLR: ReadWrite<u32>),
        /// Interrupt Priority Mask Register.
        (0x0004 => PMR: ReadWrite<u32>),
        /// Binary Point Register.
        (0x0008 => BPR: ReadWrite<u32>),
        /// Interrupt Acknowledge Register.
        (0x000c => IAR: ReadOnly<u32>),
        /// End of Interrupt Register.
        (0x0010 => EOIR: WriteOnly<u32>),
        /// Running Priority Register.
        (0x0014 => RPR: ReadOnly<u32>),
        /// Highest Priority Pending Interrupt Register.
        (0x0018 => HPPIR: ReadOnly<u32>),
        (0x001c => _reserved_1),
        /// CPU Interface Identification Register.
        (0x00fc => IIDR: ReadOnly<u32>),
        (0x0100 => _reserved_2),
        /// Deactivate Interrupt Register.
        (0x1000 => DIR: WriteOnly<u32>),
        (0x1004 => @END),
    }
}

pub struct GicDistributor {
    base: NonNull<GicDistributorRegs>,
    max_irqs: usize,
}

unsafe impl Send for GicDistributor {}
unsafe impl Sync for GicDistributor {}

pub struct GicCpuInterface {
    base: NonNull<GicCpuInterfaceRegs>,
}

unsafe impl Send for GicCpuInterface {}
unsafe impl Sync for GicCpuInterface {}

impl GicDistributor {
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
            max_irqs: MAX_IRQS,
        }
    }

    const fn regs(&self) -> &GicDistributorRegs {
        unsafe { self.base.as_ref() }
    }

    pub fn cpu_nums(&self) -> usize {
        ((self.regs().TYPER.get() as usize >> 5) & 0b111) + 1
    }

    pub fn max_irqs(&self) -> usize {
        ((self.regs().TYPER.get() as usize & 0b11111) + 1) * 32
    }

    pub fn config_interrupt(&self, vector: usize, tm: TriggerMode) {
        if vector < SPI_RANGE.start || vector >= self.max_irqs {
            return;
        }

        let idx = vector >> 4;
        let mut value = self.regs().ICFGR[idx].get();
        let config_bit = ((vector & 0b1111) << 1) + 1;
        match tm {
            TriggerMode::Edge => value |= 1 << config_bit,
            TriggerMode::Level => value &= !(1 << config_bit),
        }
        self.regs().ICFGR[idx].set(value);
    }

    pub fn set_enable(&self, vector: usize, enable: bool) {
        if vector >= self.max_irqs {
            return;
        }

        let idx = vector / 32;
        let mask = 1 << (vector % 32);

        if enable {
            self.regs().ISENABLER[idx].set(mask);
        } else {
            self.regs().ICENABLER[idx].set(mask);
        }
    }

    pub fn init(&mut self) {
        // Disable all interrupts
        for idx in (0..self.max_irqs).step_by(32) {
            self.regs().ICENABLER[idx / 32].set(u32::MAX);
            self.regs().ICPENDR[idx / 32].set(u32::MAX);
        }

        // route all SPI to CPU0
        for idx in (SPI_RANGE.start..self.max_irqs).step_by(4) {
            self.regs().ITARGETSR[idx / 4].set(0x01_01_01_01);
        }

		// set SPI to Edge trigger
		for idx in SPI_RANGE.start..self.max_irqs {
			self.config_interrupt(idx, TriggerMode::Edge);
		}

		// Enable GICD
		self.regs().CTLR.set(1);
    }
}

impl GicCpuInterface {
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
        }
    }

    const fn regs(&self) -> &GicCpuInterfaceRegs {
        unsafe { self.base.as_ref() }
    }

	pub fn iar(&self) -> u32 {
		self.regs().IAR.get()
	}

	pub fn eoi(&self, iar: u32) {
		self.regs().EOIR.set(iar)
	}

	pub fn handle_irq<F: FnOnce(u32)>(&self, handler: F) {
		let iar = self.iar();
		let vector = iar & 0x3FF;
		if vector < 1020 {
			handler(vector);
			self.eoi(iar);
		} else {
			panic!("spurious interrupt ID: {}", iar);
		}
	}

	pub fn init(&self) {
		self.regs().CTLR.set(1);
		self.regs().PMR.set(0xFF);
	}
}
