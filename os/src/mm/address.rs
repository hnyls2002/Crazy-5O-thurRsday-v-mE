use core::fmt::Debug;

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
    pub fn offset(&self) -> usize {
        self.0 & (PAGE_BYTES - 1)
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

pub struct VARange {
    pub start: VirtAddr,
    pub end: VirtAddr,
}

impl Debug for VARange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VARange")
            .field("start", &self.start)
            .field("end", &self.end)
            .finish()
    }
}

impl VARange {
    pub fn new(start: VirtAddr, end: VirtAddr) -> Self {
        assert!(start <= end, "start must be smaller than end");
        VARange { start, end }
    }
}
