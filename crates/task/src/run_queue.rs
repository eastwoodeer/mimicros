use lazy_init::LazyInit;
use scheduler::SchedulerPrototype;
use spinlock::SpinNoIrq;

use crate::task::{CurrentTask, TaskInner, TaskState};
use crate::{Scheduler, TaskRef};

pub struct RunQueue {
    scheduler: Scheduler,
}

// Single global Run-queue
pub static RUN_QUEUE: LazyInit<SpinNoIrq<RunQueue>> = LazyInit::new();

impl RunQueue {
    pub fn new() -> SpinNoIrq<Self> {
        let scheduler = Scheduler::new();

        SpinNoIrq::new(Self { scheduler })
    }

    pub fn add_task(&mut self, task: TaskRef) {
        debug!("add task: {}", task.id_name());
        self.scheduler.add_task(task);
    }

    pub fn timer_tick(&mut self) {
        let current = crate::current();

        if self.scheduler.task_tick(current.as_task_ref()) {
            current.set_preempt_pending(true);
        }
    }

    pub fn preempt_resched(&mut self) {
        let current = crate::current();
        let preempt_count = current.preempt_count();

        // RUN_QUEUE.lock() will increase preempt count, if preempt_count == 1
        // the real preempt_disable_count is 0.
        if preempt_count == 1 {
            self.resched(true);
        } else {
            current.set_preempt_pending(true);
        }
    }

    pub fn resched(&mut self, preempt: bool) {
        let prev = crate::current();
        if prev.is_running() {
            prev.set_state(TaskState::Ready);
            self.scheduler.put_prev_task(prev.clone(), preempt);
        }

        let next = self.scheduler.pick_next_task().unwrap();

        self.switch_to(prev, next);
    }

    pub fn switch_to(&mut self, prev: CurrentTask, next: TaskRef) {
        trace!("context switch: {} -> {}", prev.id_name(), next.id_name());

        next.set_preempt_pending(false);
        next.set_state(TaskState::Running);

        if prev.equal(&next) {
            return;
        }

        unsafe {
            let prev_ctx = prev.get_ctx_mut();
            let next_ctx = next.get_ctx_mut();

            CurrentTask::set_current(prev, next);
            (*prev_ctx).switch_to(&*next_ctx);
        }
    }
}

pub fn init() {
    let main_task = TaskInner::new_init("main".into());
    main_task.set_state(TaskState::Running);

    RUN_QUEUE.init_by(RunQueue::new());
    CurrentTask::init_current(main_task);
}
