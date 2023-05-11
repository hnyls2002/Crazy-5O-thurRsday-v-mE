use crate::{
    kfc_sbi::timer::{get_time, CLOCK_FREQ, MSEC_PER_SEC},
    task::{exit_cur_run_next, processor::get_cur_task_arc, suspend_cur_run_next},
};

pub fn sys_exit_impl(exit_code: i32) -> ! {
    let cur_task = get_cur_task_arc().expect("exit implementation : no current task!");
    info!("In process \"{}\", pid = {}", cur_task.name, *cur_task.pid);
    info!("Application exits with code {}", exit_code);
    exit_cur_run_next();
    panic!("Unreachable in syscall exit implentation");
}

// times in ms
pub fn sys_times_impl() -> isize {
    (get_time() / (CLOCK_FREQ / MSEC_PER_SEC)) as isize
}

pub fn sys_yield_impl() -> isize {
    suspend_cur_run_next();
    0
}

pub fn sys_fork_impl() -> isize {
    todo!()
}

// the pointer is in user's address space
pub fn sys_exec_impl(path: *const u8) -> isize {
    todo!()
}

pub fn sys_waitpid_impl(pid: isize, exit_code: *mut i32) -> isize {
    todo!()
}
