use alloc::collections::BTreeMap;

use crate::{
    config::{KERNEL_STACK_SIZE, PAGE_BYTES, TRAMPOLINE_VIRT_ADDR},
    mm::{
        kernel_space::{add_kernel_stack, remove_map_area},
        MapArea, MapPerm, VPRange, VirtAddr,
    },
};

// a guard page between each task's kernel stack
fn kernel_stack_range(pid: usize) -> (VirtAddr, VirtAddr) {
    let space_size = VirtAddr(KERNEL_STACK_SIZE + PAGE_BYTES).ceil_page().0;
    (
        VirtAddr(TRAMPOLINE_VIRT_ADDR.0 - (pid + 1) * space_size + PAGE_BYTES),
        VirtAddr(TRAMPOLINE_VIRT_ADDR.0 - (pid + 1) * space_size + PAGE_BYTES + KERNEL_STACK_SIZE),
    )
}

// hold by task_struct, alloc and dealloc in kernel space
pub struct KernelStack {
    pid: usize,
}

impl KernelStack {
    pub fn sp(&self) -> usize {
        (kernel_stack_range(self.pid).1).0
    }
    pub fn new(pid: usize) -> Self {
        let range = kernel_stack_range(pid);
        let vp_range = VPRange::new(range.0, range.1);
        let kernel_stack = MapArea::new(
            vp_range,
            crate::mm::MapType::Framed(BTreeMap::new()),
            MapPerm::R | MapPerm::W,
            None,
        );
        add_kernel_stack(kernel_stack);
        KernelStack { pid }
    }
}

impl Drop for KernelStack {
    fn drop(&mut self) {
        let range = kernel_stack_range(self.pid);
        let vp_range = VPRange::new(range.0, range.1);
        remove_map_area(&vp_range);
    }
}
