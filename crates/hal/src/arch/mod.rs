cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
        pub use self::aarch64::context::TaskContext;
        pub use self::aarch64::irq::{enable_irqs, disable_irqs};
        pub use self::aarch64::irq::{local_irq_save, local_irq_restore};
        pub use self::aarch64::cpu::{current_task_ptr, set_current_task_ptr};
    } else {
        mod dummy;
        pub use self::dummy::TaskContext;
    }
}
