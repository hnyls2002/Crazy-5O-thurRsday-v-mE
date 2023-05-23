use alloc::sync::Arc;
use lazy_static::lazy_static;

use crate::app_loader::get_app_names;

use self::{
    processor::switch_to_idle,
    task_struct::{TaskStatus, TaskStruct},
};

pub mod kernel_stack;
pub mod pid_allocator;
pub mod processor;
pub mod switch;
pub mod task_context;
pub mod task_manager;
pub mod task_struct;

pub use processor::PROCESSOR;
pub use task_manager::TASK_MANAGER;

lazy_static! {
    pub static ref INIT_PROC: Arc<TaskStruct> = Arc::new(TaskStruct::new_from_elf("initproc"));
}

pub fn suspend_cur_run_next() {
    // suspend current task
    let cur_task = PROCESSOR.take_out_current().expect("no current task");
    let cur_task_ctx_ptr = cur_task.task_ctx_ptr();
    cur_task.mark_task_status(TaskStatus::Ready);
    TASK_MANAGER.add_ready_task(cur_task);

    // switch to idle
    switch_to_idle(cur_task_ctx_ptr);
}

// if normal exit, exit_code = 0
// else exit_code = -1
pub fn exit_cur_run_next(exit_code: isize) {
    // take out current task, so PROCESSOR.current will be None
    let cur_task = PROCESSOR.take_out_current().expect("no current task");
    let cur_task_ctx_ptr = cur_task.task_ctx_ptr();
    cur_task.exit_task(exit_code);
    // should manually drop cur_task
    drop(cur_task);
    switch_to_idle(cur_task_ctx_ptr)
}

pub fn task_init() {
    let name_list = get_app_names();
    info!("====================The Supported Apps====================");
    for &name in name_list.iter() {
        info!("{}", name);
    }
    info!("==========================================================");

    TASK_MANAGER.add_ready_task(INIT_PROC.clone());
}
