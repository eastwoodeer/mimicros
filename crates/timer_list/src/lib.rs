#![cfg_attr(not(test), no_std)]

extern crate alloc;

use alloc::collections::BinaryHeap;
use core::cmp::Ordering;
use core::time::Duration;

pub trait CallableEvent {
    fn callback(self, now: Duration);
}

pub struct TimerEvent<E> {
    deadline: Duration,
    event: E,
}

pub struct TimerList<E: CallableEvent> {
    events: BinaryHeap<TimerEvent<E>>,
}

impl<E> PartialOrd for TimerEvent<E> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<E> Ord for TimerEvent<E> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.deadline.cmp(&self.deadline) // Min-Heap
    }
}

impl<E> PartialEq for TimerEvent<E> {
    fn eq(&self, other: &Self) -> bool {
        self.deadline.eq(&other.deadline)
    }
}

impl<E> Eq for TimerEvent<E> {}

impl<E: CallableEvent> TimerList<E> {
    pub fn new() -> Self {
        Self {
            events: BinaryHeap::new(),
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn add(&mut self, deadline: Duration, event: E) {
        self.events.push(TimerEvent { deadline, event });
    }

    pub fn cancel<F>(&mut self, condition: F)
    where
        F: Fn(&E) -> bool,
    {
        // retain will traverse all elements in BinaryHeap, performance may down.
        self.events.retain(|e| !condition(&e.event));
    }

    pub fn expire_event(&mut self, now: Duration) -> Option<(Duration, E)> {
        if let Some(e) = self.events.peek() {
            if e.deadline <= now {
                return self.events.pop().map(|e| (e.deadline, e.event));
            }
        }
        None
    }
}
