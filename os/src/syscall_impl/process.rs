use crate::task::task_manager::exit_cur_run_next;

pub fn sys_exit_impl(exit_code: i32) -> ! {
    info!("Application exits with code {}", exit_code);
    exit_cur_run_next();
    panic!("Unreachable in syscall exit implentation");
}

pub fn sys_yield_impl() -> isize {
    todo!()
}
