use core::{
    alloc::{GlobalAlloc, Layout},
    ops::Deref,
    ptr::{null_mut, NonNull},
};

use crate::kfc_util::up_safe_cell::UPSafeCell;

use super::buddy_allocator::Heap;

// roughly implementation for `locked` heap
// use UPSafeCell to wrap Heap : Sync
pub struct UPSafeHeap(UPSafeCell<Heap>);

impl Deref for UPSafeHeap {
    type Target = UPSafeCell<Heap>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl UPSafeHeap {
    pub const fn new() -> Self {
        UPSafeHeap(UPSafeCell::new(Heap::new()))
    }
}

unsafe impl GlobalAlloc for UPSafeHeap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.exclusive_access()
            .alloc(layout)
            .ok()
            .map_or(null_mut(), |x| x.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.exclusive_access()
            .dealloc(NonNull::new_unchecked(ptr), layout);
    }
}
