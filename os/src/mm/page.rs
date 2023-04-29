use crate::config::{PAGE_MAX_OFFSET, PAGE_SIZE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Page(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame(usize);

impl Frame {
    pub fn new(x: usize) -> Self {
        if (x & PAGE_MAX_OFFSET) != 0 {
            panic!("Frame address is not aligned: {:x}", x);
        }
        Frame(x)
    }
    pub fn lower_page(&self) -> Self {
        let mut ret = self.clone();
        ret.0 -= PAGE_SIZE;
        ret
    }
}
