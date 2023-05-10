use alloc::vec::Vec;

use crate::kfc_util::up_safe_cell::UPSafeCell;
use core::ops::Deref;

struct PIDAllocatorInner {
    x: usize,
    y: Vec<usize>,
}

struct PIDAllocator {
    inner: UPSafeCell<PIDAllocatorInner>,
}

impl PIDAllocator {
    fn alloc(&self) -> usize {
        let mut inner = self.inner.exclusive_access();
        if inner.y.is_empty() {
            inner.x += 1;
            inner.x - 1
        } else {
            inner.y.pop().unwrap()
        }
    }
    fn dealloc(&self, t: usize) {
        let mut inner = self.inner.exclusive_access();
        assert!(t < inner.x, "invalid pid to dealloc");
        inner.y.push(t);
    }
}

static PID_ALLOCATOR: PIDAllocator = PIDAllocator {
    inner: UPSafeCell::new(PIDAllocatorInner {
        x: 1, // init process has pid 1
        y: Vec::new(),
    }),
};

pub struct PIDTracker(usize);

pub fn pid_alloc() -> PIDTracker {
    PIDTracker(PID_ALLOCATOR.alloc())
}

impl Deref for PIDTracker {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for PIDTracker {
    fn drop(&mut self) {
        PID_ALLOCATOR.dealloc(self.0);
    }
}
