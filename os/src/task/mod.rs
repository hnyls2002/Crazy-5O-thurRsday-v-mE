use alloc::sync::Arc;

use crate::app_loader::get_app_names;

use self::{
    processor::{switch_to_idle, take_out_current},
    task_manager::add_suspend_task,
    task_struct::{TaskStatus, TaskStruct},
};

pub mod kernel_stack;
pub mod pid_allocator;
pub mod processor;
pub mod switch;
pub mod task_context;
pub mod task_manager;
pub mod task_struct;

pub fn suspend_cur_run_next() {
    // suspend current task
    let cur_task = take_out_current().expect("no current task");
    let cur_task_ctx_ptr = cur_task.task_ctx_ptr();
    cur_task.mark_task_status(TaskStatus::Ready);
    add_suspend_task(cur_task);

    // switch to idle
    switch_to_idle(cur_task_ctx_ptr);
}

pub fn exit_cur_run_next() {
    let cur_task = take_out_current().expect("no current task");
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
    add_suspend_task(Arc::new(initproc));
}
