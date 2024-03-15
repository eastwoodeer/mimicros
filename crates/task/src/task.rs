use alloc::{boxed::Box, string::String, sync::Arc};
use core::{
    alloc::Layout,
    cell::UnsafeCell,
    mem::ManuallyDrop,
    ops::Deref,
    ptr::NonNull,
    sync::atomic::{AtomicBool, AtomicU64, AtomicU8, AtomicUsize, Ordering},
};

use hal::arch::TaskContext;
use memory_addr::{align_up, VirtAddr};

use crate::{Task, TaskRef};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new() -> Self {
        static ID_COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }

    pub const fn as_u64(&self) -> u64 {
        self.0
    }
}

pub enum TaskState {
    Running = 1,
    Ready = 2,
    Blocked = 3,
    Existed = 4,
}

impl From<u8> for TaskState {
    #[inline]
    fn from(v: u8) -> Self {
        match v {
            1 => Self::Running,
            2 => Self::Ready,
            3 => Self::Blocked,
            4 => Self::Existed,
            _ => unreachable!(),
        }
    }
}

pub struct TaskStack {
    ptr: NonNull<u8>,
    layout: Layout,
}

impl TaskStack {
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(align_up(size, 4096), 16).unwrap();
        Self {
            ptr: NonNull::new(unsafe { alloc::alloc::alloc(layout) }).unwrap(),
            layout,
        }
    }

    pub fn top(&self) -> VirtAddr {
        unsafe { core::mem::transmute(self.ptr.as_ptr().add(self.layout.size())) }
    }
}

pub struct TaskInner {
    id: TaskId,
    name: String,
    state: AtomicU8,
    is_init: bool,
    need_resched: AtomicBool,
    preempt_disable_count: AtomicUsize,
    stack: Option<TaskStack>,
    ctx: UnsafeCell<TaskContext>,
    entry: Option<*mut dyn FnOnce() -> usize>,
}

unsafe impl Send for TaskInner {}
unsafe impl Sync for TaskInner {}

impl TaskInner {
    fn new_common(name: String) -> Self {
        Self {
            id: TaskId::new(),
            name,
            state: AtomicU8::new(TaskState::Ready as u8),
            is_init: false,
            need_resched: AtomicBool::new(false),
            preempt_disable_count: AtomicUsize::new(0),
            stack: None,
            ctx: UnsafeCell::new(TaskContext::new()),
            entry: None,
        }
    }

    pub fn new_init(name: String) -> TaskRef {
        let mut t = Self::new_common(name);
        t.is_init = true;
        Arc::new(Task::new(t))
    }

    pub fn new<F>(entry: F, name: String, stack_size: usize) -> TaskRef
    where
        F: FnOnce() -> usize + Send + 'static,
    {
        let mut inner = Self::new_common(name);
        let stack = TaskStack::new(stack_size);
        inner.ctx.get_mut().init(task_entry as usize, stack.top());
        inner.entry = Some(Box::into_raw(Box::new(entry)));
        inner.stack = Some(stack);

        Arc::new(Task::new(inner))
    }

    pub fn id_name(&self) -> String {
        alloc::format!("{:?}({})", self.name, self.id.as_u64())
    }

    #[inline]
    pub fn state(&self) -> TaskState {
        self.state.load(Ordering::Acquire).into()
    }

    #[inline]
    pub fn set_state(&self, state: TaskState) {
        self.state.store(state as u8, Ordering::Release);
    }

    #[inline]
    pub fn is_ready(&self) -> bool {
        matches!(self.state(), TaskState::Ready)
    }

    #[inline]
    pub fn is_running(&self) -> bool {
        matches!(self.state(), TaskState::Running)
    }

    #[inline]
    pub fn is_blocked(&self) -> bool {
        matches!(self.state(), TaskState::Blocked)
    }

    #[inline]
    pub fn set_preempt_pending(&self, pending: bool) {
        self.need_resched.store(pending, Ordering::Release);
    }

    #[inline]
    pub fn need_resched(&self) -> bool {
        self.need_resched.load(Ordering::Acquire)
    }

    #[inline]
    pub fn disable_preempt(&self) {
        self.preempt_disable_count.fetch_add(1, Ordering::Relaxed);
    }

    #[inline]
    pub fn enable_preempt(&self) {
        if self.preempt_disable_count.fetch_sub(1, Ordering::Relaxed) == 1 {
            Self::check_preempt_pending();
        }
    }

    #[inline]
    pub fn preempt_count(&self) -> usize {
        self.preempt_disable_count.load(Ordering::Acquire)
    }

    #[inline]
    pub fn can_preempt(&self, count: usize) -> bool {
        self.preempt_disable_count.load(Ordering::Acquire) == count
    }

    pub fn check_preempt_pending() {
        let current = crate::current();
        if current.need_resched() && current.preempt_count() == 0 {
            let mut rq = crate::run_queue::RUN_QUEUE.lock();
            if current.need_resched() {
                rq.preempt_resched();
            }
        }
    }

    pub fn get_ctx_mut(&self) -> *mut TaskContext {
        self.ctx.get()
    }

    // fn test() -> Arc<Task> {
    //     TaskInner::new(test_fn, "hello".into(), 4096)
    // }

    // pub fn run() -> usize {
    //     let a = Self::test();
    // if let Some(entry) = a.inner.entry {
    //     return unsafe { Box::from_raw(entry)() };
    // entry();
    // let v = &entry;
    // v();
    // }

    //     0
    // }
}

impl Drop for TaskInner {
    fn drop(&mut self) {
        debug!("drop task: {}", self.id_name());
    }
}

pub struct CurrentTask(ManuallyDrop<TaskRef>);

impl CurrentTask {
    pub fn try_get() -> Option<Self> {
        let ptr: *const Task = hal::arch::current_task_ptr();
        if !ptr.is_null() {
            Some(Self(ManuallyDrop::new(unsafe { TaskRef::from_raw(ptr) })))
        } else {
            None
        }
    }

    pub fn get() -> Self {
        Self::try_get().expect("try to get current task failed.")
    }

    pub fn init_current(current: TaskRef) {
        let ptr = Arc::into_raw(current);
        hal::arch::set_current_task_ptr(ptr);
    }

    pub fn set_current(prev: Self, next: TaskRef) {
        let Self(v) = prev;
        // drop v
        ManuallyDrop::into_inner(v);
        let ptr = Arc::into_raw(next);
        hal::arch::set_current_task_ptr(ptr);
    }

    #[inline]
    pub fn as_task_ref(&self) -> &TaskRef {
        &self.0
    }

    #[inline]
    pub fn equal(&self, other: &TaskRef) -> bool {
        Arc::ptr_eq(&self.0, other)
    }

    pub fn clone(&self) -> TaskRef {
        self.0.deref().clone()
    }
}

impl Deref for CurrentTask {
    type Target = TaskInner;
    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

extern "C" fn task_entry() -> ! {
    let task = crate::current();

    unsafe {
        crate::run_queue::RUN_QUEUE.force_unlock();
    }

    hal::arch::enable_irqs();

    if let Some(entry) = task.entry {
        (unsafe { Box::from_raw(entry) })();
    }
    panic!("task end!");
}
