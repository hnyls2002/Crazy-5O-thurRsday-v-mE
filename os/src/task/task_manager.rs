use alloc::vec::Vec;

use crate::{app_loader::get_app_num, kfc_util::up_safe_cell::UPSafeCell};
use core::ops::Deref;
use lazy_static::lazy_static;

use super::task_info::TaskInfo;

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: UPSafeCell::new(TaskManagerInner::new_bare()),
    };
}

pub struct TaskManager {
    pub inner: UPSafeCell<TaskManagerInner>,
}

impl Deref for TaskManager {
    type Target = UPSafeCell<TaskManagerInner>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

pub struct TaskManagerInner {
    pub task_num: usize,
    pub task_list: Vec<TaskInfo>,
    pub cur_task: usize,
}

impl TaskManagerInner {
    pub fn new_bare() -> Self {
        TaskManagerInner {
            task_num: 0,
            task_list: Vec::new(),
            cur_task: 0,
        }
    }
    pub fn init_all_apps(&mut self) {
        self.task_num = get_app_num();
        for i in 0..self.task_num {
            self.task_list.push(TaskInfo::new_init(i));
        }
        todo!()
    }
}
