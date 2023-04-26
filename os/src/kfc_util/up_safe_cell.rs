use core::cell::{RefCell, RefMut};

// multiple threads : not allowed
// we just assert that UPSafeCell is Sync

pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

// unsafe impl Sync so that compiler would not check it
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    pub const fn new(item: T) -> Self {
        Self {
            inner: RefCell::new(item),
        }
    }
    pub fn exclusive_access(&self) -> RefMut<T> {
        self.inner.borrow_mut()
    }
}
