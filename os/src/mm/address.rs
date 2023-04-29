use crate::config::{PAGE_MAX_OFFSET, PAGE_SIZE};

use super::page::Frame;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VirtAddr(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(usize);

impl PhysAddr {
    pub fn new(x: usize) -> Self {
        PhysAddr(x)
    }

    pub fn floor(&self) -> Frame {
        Frame::new(self.0 & !PAGE_MAX_OFFSET)
    }

    pub fn ceil(&self) -> Frame {
        Frame::new(((self.0 - 1) & !PAGE_MAX_OFFSET) + PAGE_SIZE)
    }
}
