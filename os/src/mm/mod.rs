pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod kernel_space;
pub mod map_area;
pub mod memory_set;
pub mod mm_test;
pub mod page;
pub mod page_table;

pub use address::{PhysAddr, VARange, VirtAddr};
pub use frame_allocator::{frame_alloc, frame_allocator_init, frame_dealloc, FrameTracker};
pub use heap_allocator::{heap_init, heap_test::heap_test};
pub use kernel_space::KERNEL_SPACE;
pub use map_area::{MapArea, MapPerm, MapType};
pub use memory_set::MemorySet;
pub use page::{Frame, Page, VPRange};
pub use page_table::{PTEFlags, PageTable, PTE};

use self::{
    kernel_space::{activate_kernel_space, kernel_space_init},
    mm_test::remap_test,
};

#[no_mangle]
pub fn mm_init() {
    // buddy allocator
    heap_init();
    heap_test();

    // physical frame allocator
    frame_allocator_init();

    // kernel memory space
    kernel_space_init();
    remap_test();
    activate_kernel_space();

    // exception test code...
    // unsafe {
    //     // illegal instruction
    //     error!("mtvec: {:#X?}", mtvec::read());

    //     // page fault
    //     let addr = 0x8090_0000 as *mut usize;
    //     addr.write_volatile("Hello, world!\0".as_ptr() as usize);
    // };
}
