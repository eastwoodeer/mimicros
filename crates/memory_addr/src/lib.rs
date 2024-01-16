#![no_std]

#[derive(Copy, Clone, Debug)]
pub struct PhysAddr(usize);

impl PhysAddr {
    #[inline]
    pub const fn from(addr: usize) -> Self {
        Self(addr)
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    #[inline]
    pub const fn as_mut_ptr(self) -> *mut u8 {
        self.0 as *mut u8
    }
}

#[derive(Copy, Clone)]
pub struct VirtAddr(usize);

impl VirtAddr {
    #[inline]
    pub const fn from(addr: usize) -> Self {
        Self(addr)
    }

    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    #[inline]
    pub const fn as_ptr(self) -> *const u8 {
        self.0 as *const u8
    }

    #[inline]
    pub const fn as_mut_ptr(self) -> *mut u8 {
        self.0 as *mut u8
    }
}

impl From<usize> for PhysAddr {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for VirtAddr {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
