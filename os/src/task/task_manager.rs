use alloc::vec::Vec;

use crate::{
    app_loader::get_app_num,
    kfc_util::up_safe_cell::UPSafeCell,
    mm::VirtAddr,
    task::{switch::__switch, task_context::TaskContext},
    trap::trap_context::TrapContext,
};
use core::{cmp::min, ops::Deref};
use lazy_static::lazy_static;

use super::task_info::TaskInfo;

lazy_static! {
    static ref TASK_MANAGER: TaskManager = TaskManager {
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
    pub task_infos: Vec<TaskInfo>,
    pub cur_task: usize,
}

impl TaskManagerInner {
    pub fn new_bare() -> Self {
        TaskManagerInner {
            task_num: 0,
            task_infos: Vec::new(),
            cur_task: 0,
        }
    }
    pub fn init_all_apps(&mut self) {
        self.task_num = get_app_num();
        for i in 0..self.task_num {
            self.task_infos.push(TaskInfo::new_init(i));
        }
    }
}

pub fn init_all_apps() {
    TASK_MANAGER.exclusive_access().init_all_apps();
}

pub fn get_cur_trap_ctx_mut() -> &'static mut TrapContext {
    let cur_id = TASK_MANAGER.exclusive_access().cur_task;
    let cur_frame = TASK_MANAGER.exclusive_access().task_infos[cur_id].trap_ctx_frame;
    cur_frame.get_mut()
}

pub fn get_cur_token() -> usize {
    let task_manager = TASK_MANAGER.exclusive_access();
    task_manager.task_infos[task_manager.cur_task]
        .addr_space
        .get_satp_token()
}

pub fn task_ctx_ptr(id: usize) -> *mut TaskContext {
    (&TASK_MANAGER.exclusive_access().task_infos[id].task_ctx) as *const _ as *mut _
}

// virtual address may be continous, but physical address may not be
pub fn translate_cur_byte_buffer(buf: usize, len: usize) -> Option<Vec<&'static [u8]>> {
    let cur_id = TASK_MANAGER.exclusive_access().cur_task;
    let page_table = &TASK_MANAGER.exclusive_access().task_infos[cur_id]
        .addr_space
        .page_table;
    let mut ret = Vec::new();
    let mut cur_va = VirtAddr(buf);
    let mut rem_len = len;
    while rem_len > 0 {
        let cur_slice_len = min(cur_va.floor_page().next_page().0 - cur_va.0, rem_len);
        let cur_frame = page_table.translate_vp(cur_va.floor_page())?;
        let slice = &cur_frame.get_bytes_array_mut()
            [cur_va.get_offset()..cur_va.get_offset() + cur_slice_len];
        ret.push(slice);
        cur_va.0 += cur_slice_len;
        rem_len -= cur_slice_len;
    }
    Some(ret)
}

pub fn run_first_task() {
    info!("start running the first task!");
    let unused = TaskContext::empty();
    __switch(&unused as *const _ as *mut _, task_ctx_ptr(0));
}
