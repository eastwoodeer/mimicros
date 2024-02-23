#![no_std]

#[macro_use]
extern crate log;

pub mod bits64;

#[derive(Debug)]
pub enum PagingError {
    NoMemory,
    NotMapped,
    MappedToHugePage,
    AlreadyMapped,
    NotAligned,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum PageSize {
    Size4K = 0x1000,
    Size2M = 0x20_0000,
    Size1G = 0x4000_0000,
}

impl PageSize {
    fn is_huge(self) -> bool {
        matches!(self, Self::Size1G | Self::Size2M)
    }
}

impl From<PageSize> for usize {
    fn from(value: PageSize) -> Self {
        value as usize
    }
}

pub type PagingResult<T = ()> = Result<T, PagingError>;
