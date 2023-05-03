use crate::config::KERNEL_HEAP_SIZE;

use self::up_safe_allocator::UPSafeHeap;

pub mod buddy_allocator;
pub mod heap_test;
pub mod instrusive_linked_list;
pub mod up_safe_allocator;

// heap_allocator instance
#[global_allocator]
static HEAP_ALLOCATOR: UPSafeHeap = UPSafeHeap::new();

// heap space for kernel
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:#X?}", layout);
}

pub fn heap_init() {
    unsafe {
        HEAP_ALLOCATOR
            .exclusive_access()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE)
    };
}
