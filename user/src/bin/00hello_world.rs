#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello world from user mode!");
    println!("Hello world from user mode!");
    trace!("test trace log");
    info!("test info log");
    debug!("test debug log");
    warn!("test warn log");
    error!("test error log");
    let mut v = Vec::new();
    for i in 0..100 {
        v.push(i);
    }
    for i in 0..100 {
        assert_eq!(v[i], i);
    }
    v.reverse();
    for it in v.iter() {
        println!("{}", it);
    }
    0
}
