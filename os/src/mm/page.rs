use core::slice;

use riscv::addr::BitField;

use crate::config::{PAGE_SIZE, PPN_RANGE, PTE_NUM, SV39_INDEX_BITS, SV39_INDEX_START};

use super::page_table::PTE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(pub usize);

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
}

impl Frame {
    pub fn lower_page(&self) -> Self {
        let mut ret = self.clone();
        ret.0 -= PAGE_SIZE;
        ret
    }

    pub fn get_ppn(&self) -> usize {
        self.0.get_bits(PPN_RANGE)
    }

    pub fn get_pte_array_mut(&self) -> &mut [PTE] {
        let pa = self.0;
        unsafe { slice::from_raw_parts_mut(pa as *mut PTE, PTE_NUM) }
    }

    pub fn get_bytes_array_mut(&self) -> &mut [u8] {
        let pa = self.0;
        unsafe { slice::from_raw_parts_mut(pa as *mut u8, PAGE_SIZE) }
    }
}
