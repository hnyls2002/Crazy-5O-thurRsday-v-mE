#![allow(unused)]
use crate::syscall::{
    sys_exec, sys_exit, sys_fork, sys_getpid, sys_read, sys_times, sys_write, sys_yield,
};

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize {
    sys_yield()
}

pub fn get_time() -> isize {
    sys_times()
}

pub fn read(fd: usize, buf: &mut [u8]) -> isize {
    sys_read(fd, buf)
}

pub fn getpid() -> isize {
    sys_getpid()
}

pub fn fork() -> isize {
    sys_fork()
}

pub fn exec(path: &str) -> isize {
    sys_exec(path)
}

pub fn waitpid(pid: usize, exit_code: &mut i32) -> isize {
    loop {}
    todo!()
}

pub fn wait(exit_code: &mut i32) -> isize {
    todo!()
}
