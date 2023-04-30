pub mod address;
pub mod frame_allocator;
pub mod heap_allocator;
pub mod memory_set;
pub mod page;
pub mod page_table;

pub use address::{PhysAddr, VirtAddr};
pub use frame_allocator::{frame_alloc, frame_dealloc, FrameTracker};
pub use memory_set::{MapArea, MapPerm, MapType, MemorySet};
pub use page::{Frame, Page, VPRange};
pub use page_table::{PTEFlags, PageTable, PTE};

#[allow(unused)]
extern "C" {
    fn stext();
    fn etext();
    fn srodata();
    fn erodata();
    fn sdata();
    fn edata();
    fn sbss_with_stack();
    fn sbss();
    fn ebss();
    fn skernel();
    fn ekernel();
    fn strampoline();
}

impl MemorySet {
    pub fn new_kernel_space() -> Self {
        // TODO : trampoline to be set...
        let mut memory_set = MemorySet::new();

        // .text
        let text = MapArea::new(
            VPRange::new(VirtAddr(stext as usize), VirtAddr(etext as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::X,
        );
        memory_set.add_map_area(text);

        // .rodata
        let rodata = MapArea::new(
            VPRange::new(VirtAddr(srodata as usize), VirtAddr(erodata as usize)),
            MapType::Identical,
            MapPerm::R,
        );
        memory_set.add_map_area(rodata);

        // .data
        let data = MapArea::new(
            VPRange::new(VirtAddr(sdata as usize), VirtAddr(edata as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
        );
        memory_set.add_map_area(data);

        // .bss : including the stack part
        let bss = MapArea::new(
            VPRange::new(VirtAddr(sbss_with_stack as usize), VirtAddr(ebss as usize)),
            MapType::Identical,
            MapPerm::R | MapPerm::W,
        );
        memory_set.add_map_area(bss);

        // available memoery area

        todo!()
    }
}
