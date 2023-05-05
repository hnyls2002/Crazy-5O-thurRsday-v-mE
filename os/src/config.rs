// set the kernel's config
#![allow(dead_code)]
use core::ops::Range;

use crate::mm::VirtAddr;

pub const MEMORY_END: usize = 0x80800000; // 8 MB

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000; // 3MB
pub const BUDDY_MAX_ORDER: usize = 32; // as large as possible...

// page table
pub const PAGE_BYTES_BITS: usize = 12;
pub const PAGE_BYTES: usize = 4096;

// SV39 : VP
pub const VP_INDEX_NUM: usize = 512;
pub const VP_INDEX_BITS: usize = 9;

// SV39 : PTE
pub const PTE_FLAGS_MASK: usize = (1 << 8) - 1;
pub const PTE_PPN_RANGE: Range<usize> = 10..54;

// SV39 : PP
pub const PP_PPN_RANGE: Range<usize> = 12..56;

// Stack
pub const USER_STACK_SIZE: usize = 0x2000; // 8KB
pub const KERNEL_STACK_SIZE: usize = 0x2000; // 8KB

// trap
pub const VIRT_ADDR_MAX: VirtAddr = VirtAddr(usize::MAX);
pub const TRAMPOLINE_VIRT_ADDR: VirtAddr = VirtAddr(VIRT_ADDR_MAX.0 - PAGE_BYTES);
pub const TRAP_CTX_VIRT_ADDR: VirtAddr = VirtAddr(TRAMPOLINE_VIRT_ADDR.0 - PAGE_BYTES);

// a guard page between each task's kernel stack
pub fn kernel_stack_range(task_id: usize) -> (VirtAddr, VirtAddr) {
    let space_size = VirtAddr(KERNEL_STACK_SIZE + PAGE_BYTES).ceil_page().0;
    (
        VirtAddr(TRAMPOLINE_VIRT_ADDR.0 - task_id * space_size + PAGE_BYTES),
        VirtAddr(TRAMPOLINE_VIRT_ADDR.0 - task_id * space_size + PAGE_BYTES + KERNEL_STACK_SIZE),
    )
}
