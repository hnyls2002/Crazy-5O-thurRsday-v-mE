#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello world from user mode!");
    trace!("test trace log");
    info!("test info log");
    debug!("test debug log");
    warn!("test warn log");
    error!("test error log");
    0
}
