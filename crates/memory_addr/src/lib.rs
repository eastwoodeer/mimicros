#![no_std]

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
}

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
}
