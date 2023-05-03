#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
#[macro_use]
mod kfc_logger;

mod lang_items;
mod syscall;

pub use syscall::{sys_exit, sys_write};

#[no_mangle]
#[linkage = "weak"]
fn main() -> i32 {
    panic!("Cannot find main function in user program");
}

#[no_mangle]
#[link_section = "text.entry"]
pub extern "C" fn _start() -> ! {
    sys_exit(main());
    panic!("app should exit!");
}
