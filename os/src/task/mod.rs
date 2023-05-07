use self::task_manager::task_manager_init;

pub mod switch;
pub mod task_context;
pub mod task_info;
pub mod task_manager;

pub fn task_init() {
    task_manager_init();
}
