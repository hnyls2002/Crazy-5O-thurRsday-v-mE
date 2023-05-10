#![no_std]
#![no_main]

use user_lib::api::{get_time, yield_};

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let start_time = get_time();
    let wait_for = start_time + 3000;
    let mut cur_sum = 0 as isize;
    loop {
        let now_time = get_time();
        if now_time - start_time > cur_sum + 200 {
            cur_sum = (now_time - start_time) / 200 * 200;
            println!("Sleeping for {} ms", cur_sum);
        }
        if now_time > wait_for {
            break;
        }
        yield_();
    }
    println!("Test sleep OK!");
    0
}
