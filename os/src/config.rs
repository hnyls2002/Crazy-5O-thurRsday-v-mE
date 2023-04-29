// set the kernel's config
#![allow(dead_code)]

pub const MEMORY_END: usize = 0x80800000; // 8 MB

pub const KERNEL_HEAP_SIZE: usize = 0x30_0000; // 3MB
pub const BUDDY_MAX_ORDER: usize = 32; // as large as possible...

pub const PAGE_OFFSET_ORDER: usize = 12;
pub const PAGE_MAX_OFFSET: usize = (1 << PAGE_OFFSET_ORDER) - 1;
pub const PAGE_SIZE: usize = 1 << PAGE_OFFSET_ORDER;
