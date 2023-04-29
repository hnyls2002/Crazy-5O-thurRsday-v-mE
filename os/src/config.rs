// set the kernel's config
#![allow(dead_code)]

use core::ops::Range;

pub const MEMORY_END: usize = 0x80800000; // 8 MB

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000; // 3MB
pub const BUDDY_MAX_ORDER: usize = 32; // as large as possible...

// page table
pub const PAGE_SIZE_BITS: usize = 12;
pub const PAGE_SIZE: usize = 4096;
pub const PTE_NUM: usize = 512;

// SV39
pub const SV39_INDEX_BITS: usize = 9;
pub const SV39_INDEX_START: usize = 12;
pub const PPN_RANGE: Range<usize> = 12..56;
