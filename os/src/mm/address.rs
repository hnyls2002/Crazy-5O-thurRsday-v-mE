use crate::config::PAGE_SIZE;

use super::page::Frame;

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

impl VirtAddr {}

impl PhysAddr {
    pub fn floor_frame(&self) -> Frame {
        Frame(align_down(self.0, PAGE_SIZE))
    }
    pub fn ceil_frame(&self) -> Frame {
        Frame(align_up(self.0, PAGE_SIZE))
    }
}
