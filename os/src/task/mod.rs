use self::task_manager::task_manager_init;

pub mod kernel_stack;
pub mod pid_allocator;
pub mod processor;
pub mod switch;
pub mod task_context;
pub mod task_manager;
pub mod task_struct;

pub fn task_init() {
    task_manager_init();
}
