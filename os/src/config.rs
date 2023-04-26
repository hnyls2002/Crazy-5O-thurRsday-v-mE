// set the kernel's config
#![allow(dead_code)]
pub const KERNEL_HEAP_SIZE: usize = 0x30_0000; // 3MB
pub const BUDDY_MAX_ORDER: usize = 32; // as large as possible...
