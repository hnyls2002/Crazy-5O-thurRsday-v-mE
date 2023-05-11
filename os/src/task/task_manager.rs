// Three levels of implementation
//
// 1. TaskManagerInner's methods : do the real work
//
// 2. TaskManager's methods : call TaskManagerInner's methods, and do some synchronization
// exclusive_access() can't be called outside TaskManager!
//
// 3. Normal functions : call TaskManager's methods
// for outside use

use alloc::{collections::VecDeque, sync::Arc};

use crate::{app_loader::get_app_names, kfc_util::up_safe_cell::UPSafeCell};
use lazy_static::lazy_static;

use super::task_struct::TaskStruct;

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: UPSafeCell::new(TaskManagerInner {
            task_structs: VecDeque::new(),
        }),
    };
}

pub struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner {
    pub task_structs: VecDeque<Arc<TaskStruct>>,
}

impl TaskManagerInner {
    #[allow(dead_code)]
    pub fn init_all_apps(&mut self) {
        let app_names = get_app_names();
        for name in app_names.iter() {
            info!("loading app {} into memory", name);
            self.task_structs
                .push_back(Arc::new(TaskStruct::new_from_elf(name)));
        }
    }

    pub fn fetch(&mut self) -> Option<Arc<TaskStruct>> {
        self.task_structs.pop_front()
    }

    pub fn add(&mut self, task: Arc<TaskStruct>) {
        self.task_structs.push_back(task);
    }
}

pub fn fetch_ready_task() -> Option<Arc<TaskStruct>> {
    TASK_MANAGER.inner.exclusive_access().fetch()
}

pub fn add_suspend_task(task: Arc<TaskStruct>) {
    TASK_MANAGER.inner.exclusive_access().add(task);
}
