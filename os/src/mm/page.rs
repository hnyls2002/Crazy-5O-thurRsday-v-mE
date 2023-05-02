use core::slice;

use riscv::addr::BitField;

use crate::config::{
    PP_PPN_RANGE, PAGE_BYTES, PAGE_BYTES_BITS, VP_INDEX_BITS, VP_INDEX_NUM,
};

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
        let s = PAGE_BYTES_BITS;
        let t = VP_INDEX_BITS;
        [
            self.0.get_bits(s + 2 * t..s + 3 * t),
            self.0.get_bits(s + t..s + 2 * t),
            self.0.get_bits(s..s + t),
        ]
    }
    pub fn next_page(&self) -> Self {
        Self(self.0 + PAGE_BYTES)
    }
}

impl Frame {
    pub fn next_page(&self) -> Self {
        Self(self.0 + PAGE_BYTES)
    }

    pub fn get_ppn(&self) -> usize {
        self.0.get_bits(PP_PPN_RANGE)
    }

    pub fn get_pte_array_mut(&self) -> &'static mut [PTE] {
        let pa = self.0;
        unsafe { slice::from_raw_parts_mut(pa as *mut PTE, VP_INDEX_NUM) }
    }

    pub fn get_bytes_array_mut(&self) -> &'static mut [u8] {
        let pa = self.0;
        unsafe { slice::from_raw_parts_mut(pa as *mut u8, PAGE_BYTES) }
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
