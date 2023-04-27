#![allow(dead_code)]
use core::{
    alloc::Layout,
    cmp::{max, min},
    fmt::Debug,
    mem::size_of,
    ptr::NonNull,
    usize,
};

use crate::config::BUDDY_MAX_ORDER;

use super::instrusive_linked_list::InLinkedList;

const TYPE_ALIGN_SIZE: usize = size_of::<usize>();

pub struct Heap {
    free_list: [InLinkedList; BUDDY_MAX_ORDER],

    user: usize,  // allocated to user
    real: usize,  // actually allocated
    total: usize, // total memory
}

impl Debug for Heap {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Heap")
            .field("user", &self.user)
            .field("real", &self.real)
            .field("total", &self.total)
            .finish()
    }
}

impl Heap {
    // empty heap
    pub const fn new() -> Self {
        Self {
            free_list: [InLinkedList::new(); BUDDY_MAX_ORDER],
            user: 0,
            real: 0,
            total: 0,
        }
    }
    // add [start, end) into head
    pub unsafe fn add_to_heap(&mut self, mut start: usize, mut end: usize) {
        start = align_start(start, TYPE_ALIGN_SIZE);
        end = align_end(end, TYPE_ALIGN_SIZE);
        assert!(start <= end);

        // at least one type size
        while start + TYPE_ALIGN_SIZE <= end {
            let lowbit = lowbit(start);
            let size = min(lowbit, prev_power_of_two(end - start));
            self.total += size;

            self.free_list[size.trailing_zeros() as usize].push(start as *mut usize);
            start += size;
        }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        self.add_to_heap(start, start + size);
    }

    // alloc size : layout.size, align, type size
    // find an avaliabel : split it
    pub fn alloc(&mut self, layout: Layout) -> Result<NonNull<u8>, ()> {
        let size = get_real_size(layout);
        let class = size.trailing_zeros() as usize;

        for i in class..self.free_list.len() {
            if !self.free_list[i].is_empty() {
                // split it
                for j in (class + 1..i + 1).rev() {
                    if let Some(block) = self.free_list[j].pop() {
                        let half_len = (1 << (j - 1)) as usize;
                        self.free_list[j - 1].push((block as usize + half_len) as *mut usize);
                        self.free_list[j - 1].push(block);
                    } else {
                        return Err(());
                    }
                }
                // split done, return the block
                let res = NonNull::new(self.free_list[class].pop().unwrap() as *mut u8);
                // let res = NonNull::new(self.free_list[class].pop().unwrap() as *mut u8);
                assert!(res.is_some());
                self.user += layout.size();
                self.real += size;
                return Ok(res.unwrap());
            }
        }
        Err(())
    }

    pub fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {
        let size = get_real_size(layout);
        let class = size.trailing_zeros() as usize;

        self.free_list[class].push(ptr.as_ptr() as *mut usize);

        //merge buddy list
        let mut cur_ptr = ptr.as_ptr() as *mut usize;
        let mut cur_class = class;
        while cur_class < self.free_list.len() {
            let buddy = ((cur_ptr as usize) ^ (1 << cur_class)) as *mut usize;
            let mut flag = false;
            for block in self.free_list[cur_class].iter_mut() {
                if block.value() == buddy {
                    block.remove();
                    flag = true;
                    break;
                }
            }

            // merge avaliable
            if flag {
                self.free_list[cur_class].pop();
                cur_ptr = min(cur_ptr, buddy);
                cur_class += 1;
                self.free_list[cur_class].push(cur_ptr);
            } else {
                break;
            }
        }

        self.user -= layout.size();
        self.real -= size;
    }
}

// align the start pointer to the next multiple of s
fn align_start(x: usize, s: usize) -> usize {
    return (x + s - 1) & (!s + 1);
}

fn align_end(x: usize, s: usize) -> usize {
    return x & (!s + 1);
}

fn lowbit(x: usize) -> usize {
    return x & (!x + 1);
}

fn prev_power_of_two(x: usize) -> usize {
    1 << (size_of::<usize>() * 8 - x.leading_zeros() as usize - 1)
}

fn get_real_size(layout: Layout) -> usize {
    let size = max(
        layout.size().next_power_of_two(),
        max(layout.align(), TYPE_ALIGN_SIZE),
    );
    size
}
