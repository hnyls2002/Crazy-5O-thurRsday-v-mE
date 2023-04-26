use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

use crate::kfc_util::up_safe_cell::UPSafeCell;

use super::buddy_allocator::Heap;

// roughly implementation for `locked` heap
// use UPSafeCell to wrap Heap : Sync
pub struct UPSafeHeap {
    pub up_safe_heap: UPSafeCell<Heap>,
}

impl UPSafeHeap {
    pub const fn new() -> Self {
        UPSafeHeap {
            up_safe_heap: UPSafeCell::new(Heap::new()),
        }
    }
}

unsafe impl GlobalAlloc for UPSafeHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut heap = self.up_safe_heap.exclusive_access();
        heap.alloc(layout).unwrap_or(null_mut() as *mut usize) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut heap = self.up_safe_heap.exclusive_access();
        heap.dealloc(ptr, layout)
    }
}
