#![cfg_attr(not(test), no_std)]

extern crate alloc;

pub mod rr;

pub trait SchedulerPrototype {
    type SchedTask;

    fn init(&mut self);

    fn add_task(&mut self, task: Self::SchedTask);

    fn remove_task(&mut self, task: &Self::SchedTask) -> Option<Self::SchedTask>;

    fn pick_next_task(&mut self) -> Option<Self::SchedTask>;

    fn put_prev_task(&mut self, prev: Self::SchedTask, preempt: bool);

    fn set_priority(&mut self, task: &Self::SchedTask, priority: isize) -> bool;

    fn task_tick(&mut self, current: &Self::SchedTask) -> bool;
}
