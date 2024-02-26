use core::arch::asm;

use memory_addr::VirtAddr;

#[repr(C)]
#[derive(Debug)]
pub struct TaskContext {
    pub sp: u64,
    pub tpidr_el0: u64,
    pub x19: u64,
    pub x20: u64,
    pub x21: u64,
    pub x22: u64,
    pub x23: u64,
    pub x24: u64,
    pub x25: u64,
    pub x26: u64,
    pub x27: u64,
    pub x28: u64,
    pub x29: u64,
    pub lr: u64, // x30
}

impl TaskContext {
    pub fn new() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }

    pub fn init(&mut self, entry: usize, stack: VirtAddr) {
        self.sp = stack.as_usize() as u64;
        self.lr = entry as u64;
    }

    pub fn switch_to(&mut self, next: &Self) {
        unsafe { context_switch(self, next) }
    }
}

#[naked]
unsafe extern "C" fn context_switch(_current_task: &TaskContext, _next_task: &TaskContext) {
    asm!(
        "
        // save current task
        stp x29, x30, [x0, 12*8]
        stp x27, x28, [x0, 10*8]
        stp x25, x26, [x0, 8*8]
        stp x23, x24, [x0, 6*8]
        stp x21, x22, [x0, 4*8]
        stp x19, x20, [x0, 2*8]
        mov x19, sp
        mrs x20, tpidr_el0
        stp x19, x20, [x0, 0*8]

        // restore next task
        ldp x19, x20, [x1, 0*8]
        mov sp, x19
        msr tpidr_el0, x20
        ldp x19, x20, [x1, 2*8]
        ldp x21, x22, [x1, 4*8]
        ldp x23, x24, [x1, 6*8]
        ldp x25, x26, [x1, 8*8]
        ldp x27, x28, [x1, 10*8]
        ldp x29, x30, [x1, 12*8]

        ret",
        options(noreturn),
    )
}
