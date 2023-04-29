use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{config::MEMORY_END, kfc_util::up_safe_cell::UPSafeCell};

use super::{address::PhysAddr, page::Frame};

pub trait FrameAllocator {
    fn alloc(&mut self) -> Result<Frame, ()>;
    fn dealloc(&mut self, pp: Frame) -> Result<(), ()>;
}

pub struct StackFrameAllocator {
    top: Frame,
    bottom: Frame,
    recycled: Vec<Frame>,
}

impl StackFrameAllocator {
    pub fn new_from_addr(top_addr: PhysAddr, bottom_addr: PhysAddr) -> Self {
        StackFrameAllocator {
            recycled: Vec::new(),
            top: top_addr.floor(),
            bottom: bottom_addr.ceil(),
        }
    }
}

impl FrameAllocator for StackFrameAllocator {
    fn alloc(&mut self) -> Result<Frame, ()> {
        if !self.recycled.is_empty() {
            return Ok(self.recycled.pop().unwrap());
        }
        if self.top > self.bottom {
            let ret = self.top;
            self.top = self.top.lower_page();
            return Ok(ret);
        }
        Err(())
    }

    fn dealloc(&mut self, pp: Frame) -> Result<(), ()> {
        if pp <= self.top || self.recycled.contains(&pp) {
            return Err(());
        }
        self.recycled.push(pp);
        Ok(())
    }
}

extern "C" {
    fn ekernel();
}

lazy_static! {
    static ref FRAME_ALLOCATOR: UPSafeCell<StackFrameAllocator> =
        UPSafeCell::new(StackFrameAllocator::new_from_addr(
            PhysAddr::new(ekernel as usize),
            PhysAddr::new(MEMORY_END),
        ));
}

// RAII : Resource Acquisition Is Initialization
// get resource : frame_alloc -> a frame tracker
// release resource : drop(frame_tracker) -> fram_dealloc

pub struct FrameTracker(Frame);

impl FrameTracker {
    pub fn new(pp: Frame) -> Self {
        FrameTracker(pp)
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        frame_dealloc(self);
    }
}

pub fn frame_alloc() -> Option<FrameTracker> {
    let res = FRAME_ALLOCATOR.exclusive_access().alloc();
    if let Err(()) = res {
        panic!("Frame allocation failed!");
    }
    Some(FrameTracker(res.ok().unwrap()))
}

pub fn frame_dealloc(ft: &mut FrameTracker) {
    let res = FRAME_ALLOCATOR.exclusive_access().dealloc(ft.0);
    if res.is_err() {
        panic!("Frame deallocation failed!");
    }
}
