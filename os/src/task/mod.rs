pub mod task_context;
pub mod task_info;
pub mod task_manager;

pub use self::task_manager::TASK_MANAGER;

pub fn init() {
    TASK_MANAGER.exclusive_access().init_all_apps();
}
