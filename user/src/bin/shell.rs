#![no_std]
#![no_main]

use alloc::string::String;
use user_lib::console::getchar;

#[macro_use]
extern crate user_lib;

extern crate alloc;

const LF: u8 = 10; // \n
const CR: u8 = 13; // \r
const BS: u8 = 8; // backspace
const DL: u8 = 127; // delete
const CC: u8 = 3; // Ctrl+C

const SHELL: &str = "\x1b[1;32mzck@zck-A7S\x1b[0m:\x1b[34m~\x1b[0m$ ";

#[no_mangle]
pub fn main() -> isize {
    let mut line = String::new();
    print!("{}", SHELL);
    loop {
        let c = getchar();
        match c {
            CC => break,
            LF | CR => {
                println!("");
                print!("{}", SHELL);
                println!("The last input line is '{}'", line);
                line.clear();
                print!("{}", SHELL);
            }
            BS | DL => {
                if !line.is_empty() {
                    print!("{}", BS as char); // backspace
                    print!(" "); // clear the last char
                    print!("{}", BS as char); // backspace
                    line.pop();
                }
            }
            _ => {
                print!("{}", c as char);
                line.push(c as char);
            }
        }
    }
    0
}
