use self::{
    fd::sys_write_impl,
    process::{sys_exit_impl, sys_times_impl, sys_yield_impl},
};

mod fd;
mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_TIMES: usize = 153;

pub fn syscall_dispathcer(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write_impl(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit_impl(args[0] as i32),
        SYSCALL_YIELD => sys_yield_impl(),
        SYSCALL_TIMES => sys_times_impl(),
        _ => panic!("unsupported syscall id: {}", id),
    }
}
