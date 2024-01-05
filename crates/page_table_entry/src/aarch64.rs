use crate::MemoryAttr;
use memory_addr::PhysAddr;

#[repr(u64)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MemoryType {
    Device = 0,
    Normal = 1,
}

impl From<MemoryAttr> for DescriptorAttr {
    fn from(memory_attr: MemoryAttr) -> Self {
        let mut attr = if memory_attr.contains(MemoryAttr::DEVICE) {
            Self::from_memory_type(MemoryType::Device)
        } else {
            Self::from_memory_type(MemoryType::Normal)
        };

        if memory_attr.contains(MemoryAttr::READ) {
            attr |= Self::VALID;
        }

        if !memory_attr.contains(MemoryAttr::WRITE) {
            attr |= Self::AP_RO;
        }

        if memory_attr.contains(MemoryAttr::USER) {
            attr |= Self::AP_EL0 | Self::PXN;

            if !memory_attr.contains(MemoryAttr::EXECUTE) {
                attr |= Self::UXN;
            }
        } else {
            attr |= Self::UXN;
            if !memory_attr.contains(MemoryAttr::EXECUTE) {
                attr |= Self::PXN;
            }
        }

        attr
    }
}

bitflags::bitflags! {
    #[derive(Debug)]
    pub struct DescriptorAttr: u64 {
        /// Whether this descriptor is valid
        const VALID      = 1 << 0;
        /// This descriptor gives the address of the next page table or a 4 kB page
        const NON_BLOCK  = 1 << 1;
        /// Memory attributes index
        const ATTR_INDEX = 0b111 << 2;
        /// Non-secure bit. For memory accesses from Secure state, specifies whether the output address is in the Secure or Non-secure address map
        const NS         = 1 << 5;
        /// Access permission: accessible from EL0
        const AP_EL0     = 1 << 6;
        /// Access permission: read-only
        const AP_RO      = 1 << 7;
        /// Shareability: Inner Shareable (otherwise Outer Shareable)
        const INNER      = 1 << 8;
        /// Shareability: Inner or Outer Shareable (otherwise Non-Shareable)
        const SHAREABLE  = 1 << 9;
        /// Access flag
        const AF         = 1 << 10;
        /// Non-global flag
        const NG         = 1 << 11;
        /// A hint bit indicating that the translation table entry is one of a contiguous set of entries, that might be cached in a single TLB entry
        const CONT       = 1 << 52;
        /// The Privileged execute-never field
        const PXN        = 1 << 53;
        /// The Execute-never or Unprivileged execute-never field
        const UXN        = 1 << 54;
    }
}

impl DescriptorAttr {
    const ATTR_INDEX_MASK: u64 = 0x111_00;

    // 1 for Normal memory, 0 for device memory
    const fn from_memory_type(memory_type: MemoryType) -> Self {
        let mut bits: u64 = (memory_type as u64) << 2;
        if matches!(memory_type, MemoryType::Normal) {
            bits |= Self::INNER.bits() | Self::SHAREABLE.bits();
        }
        Self::from_bits_retain(bits)
    }
}

#[derive(Copy, Clone)]
pub struct PTE(u64);

impl PTE {
    const ADDR_MASK: usize = 0x0000_FFFF_FFFF_F000; // 12..48

    pub const fn empty() -> Self {
        Self(0)
    }
}

impl PTE {
    pub fn new_page(paddr: PhysAddr, attr: MemoryAttr, is_huge: bool) -> Self {
        let mut a: DescriptorAttr = DescriptorAttr::from(attr) | DescriptorAttr::AF;
        if !is_huge {
            a |= DescriptorAttr::NON_BLOCK;
        }

        Self(a.bits() | (paddr.as_usize() & Self::ADDR_MASK) as u64)
    }

    pub fn new_table(paddr: PhysAddr) -> Self {
        let a: DescriptorAttr = DescriptorAttr::NON_BLOCK | DescriptorAttr::VALID;
        Self(a.bits() | (paddr.as_usize() & Self::ADDR_MASK) as u64)
    }
}
