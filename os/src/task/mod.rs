use self::task_manager::init_all_apps;

pub mod switch;
pub mod task_context;
pub mod task_info;
pub mod task_manager;

pub fn task_init() {
    init_all_apps();
}
