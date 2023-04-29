use super::page::{Frame, Page};

pub struct PTE(usize);

pub struct PageTable {
    entry: Frame,
}

impl PageTable {
    pub fn map_one(vp: Page, pp: Frame) -> Result<(), ()> {
        todo!()
    }
    pub fn unmap_one(vp: Page) -> Result<(), ()> {
        todo!()
    }
}
