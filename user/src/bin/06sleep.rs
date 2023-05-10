#![no_std]
#![no_main]

use user_lib::api::{get_time, yield_};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let current_timer = get_time();
    let wait_for = current_timer + 3000;
    while get_time() < wait_for {
        // println!("now time: {}", sys_times());
        yield_();
    }
    println!("Test sleep OK!");
    0
}
