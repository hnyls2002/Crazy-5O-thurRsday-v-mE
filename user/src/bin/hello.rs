#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    println!("Hello world from user mode!");
    let mut v = Vec::new();
    for i in 0..100 {
        v.push(i);
    }
    for i in 0..100 {
        assert_eq!(v[i], i);
    }
    v.reverse();
    for i in 0..100 {
        assert_eq!(v[i], 100 - i - 1);
    }
    0
}
