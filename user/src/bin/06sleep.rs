#![no_std]
#![no_main]

use user_lib::syscall::{sys_times, sys_yield};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let current_timer = sys_times();
    let wait_for = current_timer + 3000;
    while sys_times() < wait_for {
        // println!("now time: {}", sys_times());
        sys_yield();
    }
    println!("Test sleep OK!");
    0
}
