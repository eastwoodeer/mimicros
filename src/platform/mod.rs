cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
    }
}

cfg_if::cfg_if! {
	if #[cfg(all(target_arch = "aarch64", platform_family = "aarch64-qemu-virt"))] {
		mod qemu_aarch64;
		pub use self::qemu_aarch64::*;
	}
}



