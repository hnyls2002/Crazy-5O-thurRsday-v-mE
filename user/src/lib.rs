#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

#[macro_use]
pub mod console;
#[macro_use]
mod kfc_logger;

mod lang_items;
pub mod syscall;
mod up_safe_cell;
mod user_heap;

pub use syscall::{sys_exit, sys_write};

use crate::user_heap::heap_init;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    heap_init();
    sys_exit(main());
    panic!("app should exit!");
}

// a fake main function for user program
// when building user program, compiler should find another main in "user/src/bin/xxx.rs"
// so a program's main function should have the attribute "[no_mangle]"
#[no_mangle]
#[linkage = "weak"]
// we don't want this "main" be linked if the user program has its own "main"
fn main() -> i32 {
    panic!("Cannot find main function in user program");
}
