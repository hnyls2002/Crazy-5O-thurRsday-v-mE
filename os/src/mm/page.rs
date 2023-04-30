use core::slice;

use riscv::addr::BitField;

use crate::config::{PAGE_SIZE, PPN_RANGE, PTE_NUM, SV39_INDEX_BITS, SV39_INDEX_START};

use super::{VirtAddr, PTE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(pub usize);

impl Into<Frame> for Page {
    fn into(self) -> Frame {
        Frame(self.0)
    }
}

impl Into<Page> for Frame {
    fn into(self) -> Page {
        Page(self.0)
    }
}

impl Page {
    pub fn get_indices(&self) -> [usize; 3] {
        let s = SV39_INDEX_START;
        let t = SV39_INDEX_BITS;
        [
            self.0.get_bits(s..s + t),
            self.0.get_bits(s + t..s + 2 * t),
            self.0.get_bits(s + 2 * t..s + 3 * t),
        ]
    }
    pub fn next_page(&self) -> Self {
        Self(self.0 + PAGE_SIZE)
    }
}

impl Frame {
    pub fn lower_page(&self) -> Self {
        Self(self.0 - PAGE_SIZE)
    }

    pub fn get_ppn(&self) -> usize {
        self.0.get_bits(PPN_RANGE)
    }

    pub fn get_pte_array_mut(&self) -> &'static mut [PTE] {
        let pa = self.0;
        unsafe { slice::from_raw_parts_mut(pa as *mut PTE, PTE_NUM) }
    }

    pub fn get_bytes_array_mut(&self) -> &'static mut [u8] {
        let pa = self.0;
        unsafe { slice::from_raw_parts_mut(pa as *mut u8, PAGE_SIZE) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VPRange {
    pub start: Page,
    pub end: Page,
}

pub struct Iter {
    pub cur: Page,
    pub end: Page,
}

impl VPRange {
    pub fn new(start_addr: VirtAddr, end_addr: VirtAddr) -> Self {
        Self {
            start: start_addr.floor_page(),
            end: end_addr.ceil_page(),
        }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            cur: self.start,
            end: self.end,
        }
    }
}

impl Iter {
    pub fn value(&self) -> Page {
        self.cur
    }
}

impl Iterator for Iter {
    type Item = Iter;

    // as it.cur is start --> return the current value
    // then move &mut self to next
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            return None;
        }
        let ret = Iter {
            cur: self.cur,
            end: self.end,
        };
        self.cur = self.cur.next_page();
        Some(ret)
    }
}
