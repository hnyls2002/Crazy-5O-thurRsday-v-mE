// instrusive linked list : only the pointer is stored in the node
// Amazing linux world!

#![allow(dead_code)]

use core::ptr;

// InLinkedList : just works as an entry
// each node inside is a usize
// usize can be viewed as a pointer to another usize
#[derive(Clone, Copy)]
pub struct InLinkedList {
    head: *mut usize,
}

pub struct Iter {
    curr: *mut usize,
}

pub struct IterMut {
    // why prev : delete the current node
    prev: *mut usize,
    curr: *mut usize,
}

pub struct IterMutNode {
    prev: *mut usize,
    curr: *mut usize,
}

impl IterMutNode {
    pub fn remove(&self) -> *mut usize {
        unsafe { *self.prev = *self.curr };
        self.curr
    }
    pub fn value(&self) -> *mut usize {
        self.curr
    }
}

impl Iterator for Iter {
    type Item = *mut usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_null() {
            None
        } else {
            let ret = self.curr;
            self.curr = unsafe { *self.curr as *mut usize };
            Some(ret)
        }
    }
}

impl Iterator for IterMut {
    type Item = IterMutNode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr.is_null() {
            None
        } else {
            let ret = IterMutNode {
                prev: self.prev,
                curr: self.curr,
            };
            self.prev = self.curr;
            self.curr = unsafe { *self.curr as *mut usize };
            Some(ret)
        }
    }
}

impl InLinkedList {
    pub const fn new() -> Self {
        InLinkedList {
            head: ptr::null_mut(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn push(&mut self, item: *mut usize) {
        unsafe { *item = self.head as usize };
        self.head = item;
    }

    pub fn pop(&mut self) -> Option<*mut usize> {
        match self.is_empty() {
            true => None,
            false => {
                let ret = self.head;
                self.head = unsafe { *ret } as *mut usize;
                Some(ret)
            }
        }
    }

    pub fn iter(&self) -> Iter {
        Iter { curr: self.head }
    }

    pub fn iter_mut(&self) -> IterMut {
        IterMut {
            prev: &self.head as *const *mut usize as *mut usize,
            curr: self.head,
        }
    }
}
