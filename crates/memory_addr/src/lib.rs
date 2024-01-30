#![no_std]

use core::ops::{Add, AddAssign};

#[inline]
pub fn is_aligned(addr: usize, align: usize) -> bool {
    addr & (align - 1) == 0
}

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

    #[inline]
    pub fn is_aligned<T>(self, align: T) -> bool
    where
        T: Into<usize>,
    {
        is_aligned(self.0, align.into())
    }
}

#[derive(Copy, Clone, Debug)]
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

