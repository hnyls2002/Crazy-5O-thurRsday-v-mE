// Three levels of implementation
//
// 1. TaskManagerInner's methods : do the real work
//
// 2. TaskManager's methods : call TaskManagerInner's methods, and do some synchronization
// exclusive_access() can't be called outside TaskManager!
//
// 3. Normal functions : call TaskManager's methods
// for outside use

use alloc::vec::Vec;

use crate::{
    app_loader::get_app_num,
    kfc_sbi::sbi_shutdown,
    kfc_util::up_safe_cell::UPSafeCell,
    mm::{Frame, Page, VirtAddr},
    task::{switch::__switch, task_context::TaskContext, task_struct::TaskStatus},
    trap::trap_context::TrapContext,
};
use core::cmp::min;
use lazy_static::lazy_static;

use super::task_struct::TaskStruct;

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
        inner: UPSafeCell::new(TaskManagerInner::new_bare()),
    };
}

pub struct TaskManager {
    inner: UPSafeCell<TaskManagerInner>,
}

// impl Deref for TaskManager {
//     type Target = UPSafeCell<TaskManagerInner>;

//     fn deref(&self) -> &Self::Target {
//         &self.inner
//     }
// }

pub struct TaskManagerInner {
    pub task_num: usize,
    pub task_structs: Vec<TaskStruct>,
    pub cur_task: Option<usize>,
}

impl TaskManagerInner {
    pub fn new_bare() -> Self {
        TaskManagerInner {
            task_num: 0,
            task_structs: Vec::new(),
            cur_task: None,
        }
    }
    pub fn init_all_apps(&mut self) {
        self.task_num = get_app_num();
        for i in 0..self.task_num {
            self.task_structs.push(TaskStruct::new_init(i));
        }
    }

    pub fn find_next_ready(&self) -> Option<usize> {
        let start_id = self.cur_task.map_or(0, |cur_id| cur_id + 1);
        for i in 0..self.task_num {
            let possible_next = (start_id + i) % self.task_num;
            if self.task_structs[possible_next].status == TaskStatus::Ready {
                return Some(possible_next);
            }
        }
        None
    }

    pub fn mark_task_status(&mut self, id: usize, status: TaskStatus) {
        self.task_structs[id].status = status;
        if status == TaskStatus::Running {
            self.cur_task = Some(id);
        } else {
            self.cur_task = None;
        }
    }
}

impl TaskManager {
    pub fn init_all_apps(&self) {
        self.inner.exclusive_access().init_all_apps();
    }

    pub fn cur_id(&self) -> usize {
        self.inner
            .exclusive_access()
            .cur_task
            .expect("No Task is Running!")
    }

    pub fn get_token(&self, id: usize) -> usize {
        self.inner.exclusive_access().task_structs[id]
            .addr_space
            .get_satp_token()
    }

    pub fn task_ctx_ptr(&self, id: usize) -> *mut TaskContext {
        &self.inner.exclusive_access().task_structs[id].task_ctx as *const _ as *mut _
    }

    pub fn trap_ctx_mut(&self, id: usize) -> &'static mut TrapContext {
        self.inner.exclusive_access().task_structs[id]
            .trap_ctx_frame
            .get_mut()
    }

    pub fn translate_vp(&self, id: usize, vp: Page) -> Option<Frame> {
        self.inner.exclusive_access().task_structs[id]
            .addr_space
            .page_table
            .translate_vp(vp)
    }

    pub fn mark_task_status(&self, id: usize, status: TaskStatus) {
        self.inner.exclusive_access().mark_task_status(id, status);
    }

    pub fn find_next_ready(&self) -> Option<usize> {
        self.inner.exclusive_access().find_next_ready()
    }
}

/// load all apps and init task manager
pub fn task_manager_init() {
    TASK_MANAGER.init_all_apps();
}

pub fn get_cur_trap_ctx_mut() -> &'static mut TrapContext {
    let cur_id = TASK_MANAGER.cur_id();
    TASK_MANAGER.trap_ctx_mut(cur_id)
}

pub fn get_cur_token() -> usize {
    let cur_id = TASK_MANAGER.cur_id();
    TASK_MANAGER.get_token(cur_id)
}

// virtual address may be continous, but physical address may not be
pub fn translate_cur_byte_buffer_mut(buf: usize, len: usize) -> Option<Vec<&'static mut [u8]>> {
    let cur_id = TASK_MANAGER.cur_id();
    let mut ret = Vec::new();
    let mut cur_va = VirtAddr(buf);
    let mut rem_len = len;
    while rem_len > 0 {
        let cur_slice_len = min(cur_va.floor_page().next_page().0 - cur_va.0, rem_len);
        let cur_frame = TASK_MANAGER.translate_vp(cur_id, cur_va.floor_page())?;
        let slice = &mut cur_frame.get_bytes_array_mut()
            [cur_va.get_offset()..cur_va.get_offset() + cur_slice_len];
        ret.push(slice);
        cur_va.0 += cur_slice_len;
        rem_len -= cur_slice_len;
    }
    Some(ret)
}

pub fn run_first_task() {
    TASK_MANAGER.mark_task_status(0, TaskStatus::Running);
    let unused = TaskContext::empty();
    __switch(&unused as *const _ as *mut _, TASK_MANAGER.task_ctx_ptr(0))
}

pub fn exit_cur_run_next() {
    if let Some(next_id) = TASK_MANAGER.find_next_ready() {
        let cur_id = TASK_MANAGER.cur_id();
        let cur_task_ctx_ptr = TASK_MANAGER.task_ctx_ptr(cur_id);
        let next_task_ctx_ptr = TASK_MANAGER.task_ctx_ptr(next_id);
        TASK_MANAGER.mark_task_status(cur_id, TaskStatus::Excited);
        TASK_MANAGER.mark_task_status(next_id, TaskStatus::Running);
        __switch(cur_task_ctx_ptr, next_task_ctx_ptr);
    } else {
        // shutdown here
        sbi_shutdown(0);
    }
}

pub fn suspend_cur_run_next() {
    // info!("suspend current task and run next");
    if let Some(next_id) = TASK_MANAGER.find_next_ready() {
        let cur_id = TASK_MANAGER.cur_id();
        TASK_MANAGER.mark_task_status(cur_id, TaskStatus::Ready);
        TASK_MANAGER.mark_task_status(next_id, TaskStatus::Running);
        let cur_task_ctx_ptr = TASK_MANAGER.task_ctx_ptr(cur_id);
        let next_task_ctx_ptr = TASK_MANAGER.task_ctx_ptr(next_id);
        __switch(cur_task_ctx_ptr, next_task_ctx_ptr);
    }
}
