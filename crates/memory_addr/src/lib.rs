#![no_std]

use core::fmt;
use core::ops::{Add, AddAssign};

#[inline]
pub const fn is_aligned(addr: usize, align: usize) -> bool {
    addr & (align - 1) == 0
}

#[inline]
pub const fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

#[inline]
pub const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[derive(Copy, Clone)]
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

    #[inline]
    pub fn is_aligned<T>(self, align: T) -> bool
    where
        T: Into<usize>,
    {
        is_aligned(self.0, align.into())
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

    #[inline]
    pub fn is_aligned<T>(self, align: T) -> bool
    where
        T: Into<usize>,
    {
        is_aligned(self.0, align.into())
    }
}

impl From<usize> for PhysAddr {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<usize> for VirtAddr {
    #[inline]
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for PhysAddr {
    type Output = Self;

    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for VirtAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl AddAssign<usize> for PhysAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        *self = *self + rhs;
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("VA: {:#X}", self.0))
    }
}

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("PA: {:#X}", self.0))
    }
}
