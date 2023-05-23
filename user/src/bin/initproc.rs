#![no_std]
#![no_main]

use user_lib::api::{exec, exit, fork, wait};

#[macro_use]
extern crate user_lib;

#[no_mangle]
pub fn main() -> isize {
    if fork() == 0 {
        // init process only fork and exec "shell"
        exec("shell");
    } else {
        loop {
            let mut exit_code: i32 = 0;
            let pid = wait(&mut exit_code);
            if pid == -1 {
                println!("init process : no child process left, exiting...");
                exit(0);
            }
        }
    }
    0
}
