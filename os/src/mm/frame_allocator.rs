use alloc::vec::Vec;
use lazy_static::lazy_static;

use crate::{
    config::{MEMORY_END, PAGE_SIZE},
    kfc_util::up_safe_cell::UPSafeCell,
};

use super::{Frame, PhysAddr};

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
    pub fn new_empty() -> Self {
        StackFrameAllocator {
            top: Frame(0),
            bottom: Frame(0),
            recycled: Vec::new(),
        }
    }
    pub fn init(&mut self, top: Frame, bottom: Frame) {
        self.top = top;
        self.bottom = bottom;
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

/// ### RAII : Resource Acquisition Is Initialization
///
/// - get resource : `frame_alloc` -> `FrameTracker`
/// - release resource : `drop(FrameTracker)` -> `frame_dealloc`
/// - so no `Copy` or `Clone` traits here
pub struct FrameTracker(pub Frame);

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
    let res_frame = res.ok().unwrap();
    let bytes_array_mut = res_frame.get_bytes_array_mut();
    for i in 0..PAGE_SIZE {
        bytes_array_mut[i] = 0;
    }
    Some(FrameTracker(res_frame))
}

pub fn frame_dealloc(ft: &mut FrameTracker) {
    let res = FRAME_ALLOCATOR.exclusive_access().dealloc(ft.0);
    if res.is_err() {
        panic!("Frame deallocation failed!");
    }
}

lazy_static! {
    static ref FRAME_ALLOCATOR: UPSafeCell<StackFrameAllocator> =
        UPSafeCell::new(StackFrameAllocator::new_empty());
}

extern "C" {
    fn ekernel();
}

pub fn frame_allocator_init() {
    FRAME_ALLOCATOR.exclusive_access().init(
        PhysAddr(ekernel as usize).ceil_frame(),
        PhysAddr(MEMORY_END).floor_frame(),
    )
}
