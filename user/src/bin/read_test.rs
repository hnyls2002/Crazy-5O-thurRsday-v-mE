#![no_std]
#![no_main]

use user_lib::console::getchar;

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> i32 {
    println!("Input Test : please input a character!");
    let c = getchar();
    println!("====================================================");
    println!("The input character is \x1b[31m{}\x1b[0m", c as char);
    println!("Press any button to continue!");
    println!("====================================================");
    getchar();
    0
}
