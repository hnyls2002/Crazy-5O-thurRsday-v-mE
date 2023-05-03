// set the kernel's config
#![allow(dead_code)]
use core::ops::Range;

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

// User Stack
pub const USER_STACK_SIZE: usize = 0x2000; // 8KB
