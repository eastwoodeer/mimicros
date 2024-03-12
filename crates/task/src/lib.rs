#![cfg_attr(not(test), no_std)]

extern crate alloc;

#[macro_use]
extern crate log;

use alloc::sync::Arc;

mod run_queue;
mod task;
// mod timer;

use task::{CurrentTask, TaskInner};

const MAX_TIME_SLICE: usize = 5;
pub type Task = scheduler::rr::RRTask<TaskInner, MAX_TIME_SLICE>;
pub type Scheduler = scheduler::rr::RRScheduler<TaskInner, MAX_TIME_SLICE>;

pub type TaskRef = Arc<Task>;

pub fn current() -> CurrentTask {
    CurrentTask::get()
}

#[no_mangle]
pub fn __preempt_guard_enable_preempt() {
    if let Some(current) = CurrentTask::try_get() {
        current.enable_preempt();
    }
}

#[no_mangle]
pub fn __preempt_guard_disable_preempt() {
    if let Some(current) = CurrentTask::try_get() {
        current.disable_preempt();
    }
}

pub fn init_scheduler() {
    info!("init scheduler");
    run_queue::init();

    let task1 = TaskInner::new(
        || {
            info!("task 1");

            loop {
                run_queue::RUN_QUEUE.lock().yield_current();
            }
        },
        "task1".into(),
        4096,
    );
    run_queue::RUN_QUEUE.lock().add_task(task1);

    let task2 = TaskInner::new(
        || {
            info!("task 2");
            loop {}
        },
        "task2".into(),
        4096,
    );
    run_queue::RUN_QUEUE.lock().add_task(task2);
}

pub fn on_timer_tick() {
    run_queue::RUN_QUEUE.lock().timer_tick();
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_gen_taskid() {
    //     let t1 = task::TaskId::new();
    //     let t2 = task::TaskId::new();

    //     assert_eq!(t1.as_u64(), 1);
    //     assert_eq!(t2.as_u64(), 2);
    //     assert_ne!(t1, t2);
    // }

    // #[test]
    // fn test_task_entry() {
    //     let v = task::TaskInner::run();
    //     assert_eq!(42, v);
    // }
}
