use alloc::sync::Arc;

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

pub fn suspend_cur_run_next() {
    // suspend current task
    let cur_task = PROCESSOR.take_out_current().expect("no current task");
    let cur_task_ctx_ptr = cur_task.task_ctx_ptr();
    cur_task.mark_task_status(TaskStatus::Ready);
    TASK_MANAGER.add_ready_task(cur_task);

    // switch to idle
    switch_to_idle(cur_task_ctx_ptr);
}

pub fn exit_cur_run_next() {
    let cur_task = PROCESSOR.take_out_current().expect("no current task");
    let cur_task_ctx_ptr = cur_task.task_ctx_ptr();
    cur_task.mark_task_status(TaskStatus::Excited);
    // TODO : free resources
    switch_to_idle(cur_task_ctx_ptr)
}

pub fn task_init() {
    let name_list = get_app_names();
    info!("====================The Supported Apps====================");
    for &name in name_list.iter() {
        info!("{}", name);
    }
    info!("==========================================================");
    let initproc = TaskStruct::new_from_elf("shell");
    TASK_MANAGER.add_ready_task(Arc::new(initproc));
}
