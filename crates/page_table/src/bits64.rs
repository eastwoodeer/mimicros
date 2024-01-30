extern crate alloc;

use alloc::{vec, vec::Vec};

use crate::{PageSize, PagingError, PagingResult};

use allocator::global_allocator;
use memory_addr::{PhysAddr, VirtAddr};
use page_table_entry::{aarch64::PTE, MemoryAttribute};

const PTE_ENTRY_COUNT: usize = 512;
const PAGE_SHIFT: usize = 12;

const fn pgt_l1_idx(vaddr: VirtAddr) -> usize {
    (vaddr.as_usize() >> PAGE_SHIFT) & (PTE_ENTRY_COUNT - 1)
}

const fn pgt_l2_idx(vaddr: VirtAddr) -> usize {
    (vaddr.as_usize() >> (PAGE_SHIFT + 9)) & (PTE_ENTRY_COUNT - 1)
}

const fn pgt_l3_idx(vaddr: VirtAddr) -> usize {
    (vaddr.as_usize() >> (PAGE_SHIFT + 18)) & (PTE_ENTRY_COUNT - 1)
}

const fn pgt_l4_idx(vaddr: VirtAddr) -> usize {
    (vaddr.as_usize() >> (PAGE_SHIFT + 27)) & (PTE_ENTRY_COUNT - 1)
}

pub struct PageTable64 {
    root_paddr: PhysAddr,
    used_pages: Vec<PhysAddr>,
}

pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    VirtAddr::from(paddr.as_usize() + 0xFFFF_0000_0000_0000)
}

pub fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    PhysAddr::from(vaddr.as_usize() - 0xFFFF_0000_0000_0000)
}

impl PageTable64 {
    pub fn alloc_table() -> PagingResult<PhysAddr> {
        if let Ok(vaddr) = global_allocator().alloc_pages(1) {
            let paddr = virt_to_phys(vaddr.into());
            // let ptr = paddr.as_mut_ptr();

            // debug!("{:x?}", paddr);

            Ok(paddr)
        } else {
            Err(PagingError::NoMemory)
        }
    }

    pub fn new() -> Self {
        let root_paddr = Self::alloc_table().unwrap();
        Self {
            root_paddr,
            used_pages: vec![root_paddr],
        }
    }

    fn get_table(&self, paddr: PhysAddr) -> &[PTE] {
        let ptr = phys_to_virt(paddr).as_ptr() as _;
        unsafe { core::slice::from_raw_parts(ptr, 512) }
    }

    fn get_table_mut<'a>(&mut self, paddr: PhysAddr) -> &'a mut [PTE] {
        let ptr = phys_to_virt(paddr).as_mut_ptr() as _;
        unsafe { core::slice::from_raw_parts_mut(ptr, 512) }
    }

    fn get_next_table_mut<'a>(&mut self, entry: &mut PTE) -> PagingResult<&'a mut [PTE]> {
        if !entry.is_valid() {
            Err(PagingError::NotMapped)
        } else if entry.is_huge() {
            Err(PagingError::MappedToHugePage)
        } else {
            Ok(self.get_table_mut(entry.paddr()))
        }
    }

    fn get_next_table_mut_or_create<'a>(&mut self, entry: &mut PTE) -> PagingResult<&'a mut [PTE]> {
        if entry.is_unused() {
            let paddr = Self::alloc_table()?;
            self.used_pages.push(paddr);
            *entry = PTE::new_table(paddr);
            Ok(self.get_table_mut(paddr))
        } else {
            self.get_next_table_mut(entry)
        }
    }

    pub fn get_entry_mut_or_create(
        &mut self,
        vaddr: VirtAddr,
        page_size: PageSize,
    ) -> PagingResult<&mut PTE> {
        let l4_table = self.get_table_mut(self.root_paddr);
        let l4_pte = &mut l4_table[pgt_l4_idx(vaddr)];

        let l3_table = self.get_next_table_mut_or_create(l4_pte)?;
        let l3_pte = &mut l3_table[pgt_l3_idx(vaddr)];

        if page_size == PageSize::Size1G {
            return Ok(l3_pte);
        }

        let l2_table = self.get_next_table_mut_or_create(l3_pte)?;
        let l2_pte = &mut l2_table[pgt_l2_idx(vaddr)];

        if page_size == PageSize::Size2M {
            return Ok(l2_pte);
        }

        let l1_table = self.get_next_table_mut_or_create(l2_pte)?;
        let l1_pte = &mut l1_table[pgt_l1_idx(vaddr)];

        Ok(l1_pte)
    }

    pub fn get_entry_mut(&mut self, vaddr: VirtAddr) -> PagingResult<(&mut PTE, PageSize)> {
        let l4_table = self.get_table_mut(self.root_paddr);
        let l4_pte = &mut l4_table[pgt_l4_idx(vaddr)];

        let l3_table = self.get_next_table_mut(l4_pte)?;
        let l3_pte = &mut l3_table[pgt_l3_idx(vaddr)];

        if l3_pte.is_huge() {
            return Ok((l3_pte, PageSize::Size1G));
        }

        let l2_table = self.get_next_table_mut(l3_pte)?;
        let l2_pte = &mut l2_table[pgt_l2_idx(vaddr)];

        if l2_pte.is_huge() {
            return Ok((l2_pte, PageSize::Size2M));
        }

        let l1_table = self.get_next_table_mut(l2_pte)?;
        let l1_pte = &mut l1_table[pgt_l1_idx(vaddr)];

        Ok((l1_pte, PageSize::Size4K))
    }

    pub fn map(
        &mut self,
        vaddr: VirtAddr,
        paddr: PhysAddr,
        page_size: PageSize,
        attr: MemoryAttribute,
    ) -> PagingResult {
        let entry = self.get_entry_mut_or_create(vaddr, page_size)?;
        if !entry.is_unused() {
            return Err(PagingError::AlreadyMapped);
        }
        *entry = PTE::new_page(paddr, attr, page_size.is_huge());
        Ok(())
    }

    pub fn unmap(&mut self, vaddr: VirtAddr) -> PagingResult<(PhysAddr, PageSize)> {
        let (entry, page_size) = self.get_entry_mut(vaddr)?;
        if entry.is_unused() {
            return Err(PagingError::NotMapped);
        }
        let paddr = entry.paddr();
        entry.clear();

        Ok((paddr, page_size))
    }

    pub fn memmap(
        &mut self,
        vaddr: VirtAddr,
        paddr: PhysAddr,
        size: usize,
        attr: MemoryAttribute,
    ) -> PagingResult {
        if !vaddr.is_aligned(PageSize::Size4K)
            || !paddr.is_aligned(PageSize::Size4K)
            || !memory_addr::is_aligned(size, PageSize::Size4K.into())
        {
            return Err(PagingError::NotAligned);
        }

        trace!(
            "memmap({:#x?}): [{:#x?}, {:#x?}) -> [{:#x?}, {:#x?}) {:?}",
            self.root_paddr,
            vaddr,
            vaddr + size,
            paddr,
            paddr + size,
            attr
        );

        let mut size = size;
        let mut vaddr = vaddr;
        let mut paddr = paddr;
        let page_size = PageSize::Size4K;

        while size > 0 {
            self.map(vaddr, paddr, page_size, attr).inspect_err(|e| {
                error!(
                    "failed to memmap page: {:#x?}({:?}) -> {:#x?}, {:?}",
                    vaddr, page_size, paddr, e
                );
            })?;

            vaddr += page_size as usize;
            paddr += page_size as usize;
            size -= page_size as usize;
        }

        Ok(())
    }
}
