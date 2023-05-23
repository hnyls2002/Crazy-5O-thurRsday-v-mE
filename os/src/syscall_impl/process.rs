use alloc::{sync::Arc, vec::Vec};

use crate::{
    kfc_sbi::timer::{get_time, CLOCK_FREQ, MSEC_PER_SEC},
    mm::PageTable,
    task::{exit_cur_run_next, suspend_cur_run_next, PROCESSOR, TASK_MANAGER},
};

pub fn sys_exit_impl(exit_code: i32) -> ! {
    {
        let cur_task = PROCESSOR
            .current_arc()
            .expect("exit implementation : no current task!");
        info!(
            "In process \"{}\", pid = {}, exit with code {}",
            cur_task.get_name(),
            *cur_task.pid,
            exit_code
        );
        // --------cur task drop here--------
    }

    exit_cur_run_next(exit_code);

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
    forked.set_parent(Arc::downgrade(&current));
    current.add_child(forked.clone());

    let pid = *forked.pid as isize;

    // the forked process will no get system call value, it's return value is stored in a0
    let trap_ctx = forked.trap_ctx_mut();
    trap_ctx.x[10] = 0;

    TASK_MANAGER.add_ready_task(forked);
    pid
}

pub fn sys_getpid_impl() -> isize {
    *PROCESSOR.current_arc().expect("no current task!").pid as isize
}

// the pointer is in user's address space
pub fn sys_exec_impl(path: *const u8) -> isize {
    let current = PROCESSOR.current_arc().expect("no current task!");
    if let Some(res) = current.exec_from_elf(path) {
        res
    } else {
        -1
    }
}

/// NO child process has the given pid -> -1
/// The required pid is still running -> -2
pub fn sys_waitpid_impl(pid: isize, exit_code_ptr: usize) -> isize {
    let current = PROCESSOR.current_arc().expect("no current task!");
    let light_pt = PageTable {
        entry: current.pt_entry(),
        pt_frames: Vec::new(),
    };

    let exit_code_mut: &mut i32 = light_pt.get_mut(exit_code_ptr).expect("invalid pointer!");
    current.wait_task(pid, exit_code_mut)
}
