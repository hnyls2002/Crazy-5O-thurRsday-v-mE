#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
#[macro_use]
mod kfc_logger;

mod lang_items;
pub mod syscall;

pub use syscall::{sys_exit, sys_write};

// a fake main function for user program
// when building user program, compiler should find another main in "user/src/bin/xxx.rs"
// so a program's main function should have the attribute "[no_mangle]"

#[no_mangle]
#[linkage = "weak"]
// we don't want this "main" be linked if the user program has its own "main"
fn main() -> i32 {
    panic!("Cannot find main function in user program");
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    sys_exit(main());
    panic!("app should exit!");
}
