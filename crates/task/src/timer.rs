use alloc::sync::Arc;
use core::time::Duration;

use hal::time::current_time;
use lazy_init::LazyInit;
use spinlock::SpinNoIrq;
use timer_list::{ScheduledEvent, TimerList};

use crate::{run_queue::RUN_QUEUE, TaskRef};

static TIMER_LIST: LazyInit<SpinNoIrq<TimerList<ScheduledTaskEvent>>> = LazyInit::new();

struct ScheduledTaskEvent(TaskRef);

impl ScheduledEvent for ScheduledTaskEvent {
    fn callback(self, _now: core::time::Duration) {
        let mut rq = RUN_QUEUE.lock();
        self.0.set_in_timer_list(false);
        rq.unblock_task(self.0, true);
    }
}

pub fn add_timer(deadline: Duration, task: TaskRef) {
    let mut timer = TIMER_LIST.lock();
    task.set_in_timer_list(true);
    timer.add(deadline, ScheduledTaskEvent(task));
}

pub fn delete_timer(task: &TaskRef) {
    let mut timer = TIMER_LIST.lock();
    task.set_in_timer_list(false);
    timer.cancel(|t| Arc::ptr_eq(&t.0, task));
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
