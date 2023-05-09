use self::up_safe_allocator::UPSafeHeap;

mod buddy_allocator;
mod instrusive_linked_list;
mod up_safe_allocator;

const USER_HEAP_SIZE: usize = 0x8000; // 32KB
const BUDDY_MAX_ORDER: usize = 32; // as large as possible...

// user heap space
static mut USER_HEAP: [u8; USER_HEAP_SIZE] = [0; USER_HEAP_SIZE];

// heap_allocator instance
#[global_allocator]
static HEAP_ALLOCATOR: UPSafeHeap = UPSafeHeap::new();

#[alloc_error_handler]
pub fn handle_alloc_error(layout: core::alloc::Layout) -> ! {
    panic!("Heap allocation error, layout = {:#X?}", layout);
}

pub fn heap_init() {
    unsafe {
        HEAP_ALLOCATOR
            .exclusive_access()
            .init(USER_HEAP.as_ptr() as usize, USER_HEAP_SIZE)
    };
}
