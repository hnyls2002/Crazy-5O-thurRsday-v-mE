use crate::{
    kfc_sbi::timer::{get_time, CLOCK_FREQ, MSEC_PER_SEC},
    task::task_manager::{exit_cur_run_next, suspend_cur_run_next},
};

pub fn sys_exit_impl(exit_code: i32) -> ! {
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
