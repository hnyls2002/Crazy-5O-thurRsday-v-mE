use alloc::sync::Arc;

use crate::{
    kfc_sbi::timer::{get_time, CLOCK_FREQ, MSEC_PER_SEC},
    task::{exit_cur_run_next, suspend_cur_run_next, task_manager::add_suspend_task, PROCESSOR},
};

pub fn sys_exit_impl(exit_code: i32) -> ! {
    let cur_task = PROCESSOR
        .current_arc()
        .expect("exit implementation : no current task!");
    info!(
        "In process \"{}\", pid = {}",
        cur_task.get_name(),
        *cur_task.pid
    );
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
    let current = PROCESSOR.current_arc().expect("no current task!");
    let forked = Arc::new(current.fork_task_struct());
    let pid = *forked.pid as isize;

    // the forked process will no get system call value, it's return value is stored in a0
    let trap_ctx = forked.trap_ctx_mut();
    trap_ctx.x[10] = 0;

    add_suspend_task(forked);
    pid
}

pub fn sys_getpid_impl() -> isize {
    *PROCESSOR.current_arc().expect("no current task!").pid as isize
}

// the pointer is in user's address space
pub fn sys_exec_impl(path: *const u8) -> isize {
    let current = PROCESSOR.current_arc().expect("no current task!");
    if let Ok(_) = current.exec_from_elf(path) {
        0
    } else {
        -1
    }
}

pub fn sys_waitpid_impl(pid: isize, exit_code: *mut i32) -> isize {
    todo!()
}
