use alloc::{collections::VecDeque, sync::Arc};
use core::ops::Deref;
use core::sync::atomic::{AtomicIsize, Ordering};

use crate::SchedulerPrototype;

pub struct RRTask<T, const MAX_TIME_SLICE: usize> {
    inner: T,
    time_slice: AtomicIsize,
}

impl<T, const S: usize> RRTask<T, S> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            time_slice: AtomicIsize::new(S as isize),
        }
    }

    pub fn time_slice(&self) -> isize {
        self.time_slice.load(Ordering::Acquire)
    }

    pub fn reset_time_slice(&self) {
        self.time_slice.store(S as isize, Ordering::Release)
    }

    pub const fn inner(&self) -> &T {
        &self.inner
    }
}

impl<T, const S: usize> Deref for RRTask<T, S> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct RRScheduler<T, const MAX_TIME_SLICE: usize> {
    ready_queue: VecDeque<Arc<RRTask<T, MAX_TIME_SLICE>>>,
}

impl<T, const S: usize> RRScheduler<T, S> {
    pub const fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }

    pub fn name() -> &'static str {
        "Round-Robin"
    }
}

impl<T, const S: usize> SchedulerPrototype for RRScheduler<T, S> {
    type SchedTask = Arc<RRTask<T, S>>;

    fn init(&mut self) {}

    fn add_task(&mut self, task: Self::SchedTask) {
        self.ready_queue.push_back(task)
    }

    fn remove_task(&mut self, task: &Self::SchedTask) -> Option<Self::SchedTask> {
        self.ready_queue
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue.remove(idx))
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedTask> {
        self.ready_queue.pop_front()
    }

    fn put_prev_task(&mut self, prev: Self::SchedTask, preempt: bool) {
        if preempt && prev.time_slice() > 0 {
            self.ready_queue.push_front(prev);
        } else {
            prev.reset_time_slice();
            self.ready_queue.push_back(prev);
        }
    }

    fn set_priority(&mut self, _task: &Self::SchedTask, _priority: isize) -> bool {
        false
    }

    // return true if time_slice runs out.
    fn task_tick(&mut self, current: &Self::SchedTask) -> bool {
        let old_time = current.time_slice.fetch_sub(1, Ordering::Release);
        old_time <= 1
    }
}
