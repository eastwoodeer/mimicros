use core::time::Duration;

use lazy_init::LazyInit;
use spinlock::SpinNoIrq;
use timer_list::{TimerList, CallableEvent};
use crate::TaskRef;

pub static TIMER_LIST: LazyInit<SpinNoIrq<TimerList<TaskEvent>>> = LazyInit::new();

struct TaskEvent(TaskRef);

impl CallableEvent for TaskEvent {
    fn callback(self, now: core::time::Duration) {
        todo!()
    }
}

pub fn add_timer(deadline: Duration, task: TaskRef) {
    TIMER_LIST.lock().add(deadline, TaskEvent(task));
}

pub fn check_event() {
    loop {
        // let now =
    }
}

pub fn init() {
    TIMER_LIST.init_by(SpinNoIrq::new(TimerList::new()));
}
