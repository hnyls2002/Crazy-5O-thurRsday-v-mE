use self::buddy_allocator::Heap;

pub mod buddy_allocator;
pub mod instrusive_linked_list;

// heap_allocator instance
#[global_allocator]
static HEAP_ALLOCATOR: Heap = Heap::new();
