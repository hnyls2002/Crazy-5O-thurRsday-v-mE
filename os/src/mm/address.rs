use crate::config::PAGE_BYTES;

use super::{Frame, Page};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(pub usize);

pub fn align_down(addr: usize, align: usize) -> usize {
    addr & !(align - 1)
}

pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

impl VirtAddr {
    pub fn floor_page(&self) -> Page {
        Page(align_down(self.0, PAGE_BYTES))
    }
    pub fn ceil_page(&self) -> Page {
        Page(align_up(self.0, PAGE_BYTES))
    }
}

impl PhysAddr {
    pub fn floor_frame(&self) -> Frame {
        Frame(align_down(self.0, PAGE_BYTES))
    }
    pub fn ceil_frame(&self) -> Frame {
        Frame(align_up(self.0, PAGE_BYTES))
    }
}
