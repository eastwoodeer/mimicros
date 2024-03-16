use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spinlock::SpinNoIrq;

use crate::{run_queue::RUN_QUEUE, task::CurrentTask, TaskRef};

pub struct WaitQueue {
    queue: SpinNoIrq<VecDeque<TaskRef>>,
}

impl WaitQueue {
    pub fn new() -> Self {
        Self {
            queue: SpinNoIrq::new(VecDeque::new()),
        }
    }

    fn cancel_event(&self, current: CurrentTask) {
        if current.in_wait_queue() {
            self.queue
                .lock()
                .retain(|t| Arc::ptr_eq(current.as_task_ref(), t));
            current.set_in_wait_queue(false);
        }
    }

    fn wait(&self) {
        RUN_QUEUE.lock().block_current(|task| {
            task.set_in_wait_queue(true);
            self.queue.lock().push_back(task);
        });
        self.cancel_event(crate::current());
    }

    fn wait_until<F>(&self, condition: F)
    where
        F: Fn() -> bool,
    {
        loop {
            let mut rq = RUN_QUEUE.lock();
            if condition() {
                break;
            }

            rq.block_current(|task| {
                task.set_in_wait_queue(true);
                self.queue.lock().push_back(task);
            });
        }
        self.cancel_event(crate::current())
    }

    pub fn notify_one(&self, resched: bool) -> bool {
        let mut rq = RUN_QUEUE.lock();
        let mut wq = self.queue.lock();
        if let Some(task) = wq.pop_front() {
            task.set_in_wait_queue(false);
            rq.unblock_task(task, resched);
            true
        } else {
            false
        }
    }

    pub fn notify_task(&self, task: &TaskRef, resched: bool) -> bool {
        let mut rq = RUN_QUEUE.lock();
        let mut wq = self.queue.lock();
        if let Some(index) = wq.iter().position(|t| Arc::ptr_eq(t, task)) {
            task.set_in_wait_queue(false);
            rq.unblock_task(wq.remove(index).unwrap(), resched);
            true
        } else {
            false
        }
    }

    pub fn notify_all(&self, resched: bool) {
        let mut rq = RUN_QUEUE.lock();
        while let Some(task) = self.queue.lock().pop_front() {
            task.set_in_wait_queue(false);
            rq.unblock_task(task, resched);
        }
    }
}
