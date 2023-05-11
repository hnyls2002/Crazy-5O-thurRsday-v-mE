use self::{
    fs::{sys_read_impl, sys_write_impl},
    process::{
        sys_exec_impl, sys_exit_impl, sys_fork_impl, sys_getpid_impl, sys_times_impl,
        sys_waitpid_impl, sys_yield_impl,
    },
};

mod fs;
mod process;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_TIMES: usize = 153;
const SYSCALL_READ: usize = 63;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_GETPID: usize = 172;

pub fn syscall_dispathcer(id: usize, args: [usize; 3]) -> isize {
    match id {
        SYSCALL_WRITE => sys_write_impl(args[0], args[1] as *const u8, args[2]),
        SYSCALL_EXIT => sys_exit_impl(args[0] as i32),
        SYSCALL_YIELD => sys_yield_impl(),
        SYSCALL_TIMES => sys_times_impl(),
        SYSCALL_READ => sys_read_impl(args[0], args[1] as *mut u8, args[2]),
        SYSCALL_FORK => sys_fork_impl(),
        SYSCALL_EXEC => sys_exec_impl(args[0] as *const u8),
        SYSCALL_WAITPID => sys_waitpid_impl(args[0] as isize, args[1] as *mut i32),
        SYSCALL_GETPID => sys_getpid_impl(),
        _ => panic!("unsupported syscall id: {}", id),
    }
}
