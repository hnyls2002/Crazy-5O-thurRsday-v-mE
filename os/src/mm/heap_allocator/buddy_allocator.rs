use crate::config::BUDDY_MAX_ORDER;

use super::instrusive_linked_list::InLinkedList;

pub struct Heap {
    free_list: [InLinkedList; BUDDY_MAX_ORDER],

    user: usize,  // allocated to user
    real: usize,  // actually allocated
    total: usize, // total memory
}

impl Heap {
    pub const fn new() -> Self {
        todo!()
    }
}
