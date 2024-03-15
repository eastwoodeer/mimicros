use core::time::Duration;

use hal::time::current_time;
use lazy_init::LazyInit;
use spinlock::SpinNoIrq;
use timer_list::{CallableEvent, TimerList};

use crate::TaskRef;

static TIMER_LIST: LazyInit<SpinNoIrq<TimerList<TaskEvent>>> = LazyInit::new();

struct TaskEvent(TaskRef);

impl CallableEvent for TaskEvent {
    fn callback(self, _now: core::time::Duration) {
        crate::run_queue::RUN_QUEUE
            .lock()
            .unblock_task(self.0, true);
    }
}

pub fn add_timer(deadline: Duration, task: TaskRef) {
    TIMER_LIST.lock().add(deadline, TaskEvent(task));
}

pub fn check_event() {
    loop {
        let now = current_time();
        let event = TIMER_LIST.lock().expire_event(now);
        if let Some((_deadline, event)) = event {
            event.callback(now);
        } else {
            break;
        }
    }
}

pub fn init() {
    TIMER_LIST.init_by(SpinNoIrq::new(TimerList::new()));
}
