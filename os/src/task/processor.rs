use core::cmp::min;

use alloc::{sync::Arc, vec::Vec};

use crate::{kfc_util::up_safe_cell::UPSafeCell, mm::VirtAddr, trap::trap_context::TrapContext};

use super::{
    switch::__switch,
    task_context::TaskContext,
    task_manager::fetch_ready_task,
    task_struct::{TaskStatus, TaskStruct},
};

pub struct ProcessorInner {
    current: Option<Arc<TaskStruct>>,
    /// idle control flow in kernel boot stack
    idle_task_ctx: TaskContext,
}

impl ProcessorInner {
    fn idle_task_ctx_ptr(&self) -> *mut TaskContext {
        &self.idle_task_ctx as *const _ as *mut _
    }
}

pub struct Processor {
    inner: UPSafeCell<ProcessorInner>,
}

static PROCESSOR: Processor = Processor {
    inner: UPSafeCell::new(ProcessorInner {
        current: None,
        idle_task_ctx: TaskContext::empty(),
    }),
};

impl Processor {
    fn satp_token(&self) -> usize {
        self.inner
            .exclusive_access()
            .current
            .as_ref()
            .expect("no current task")
            .token
    }

    fn idle_task_ctx_ptr(&self) -> *mut TaskContext {
        self.inner.exclusive_access().idle_task_ctx_ptr()
    }

    fn trap_ctx_mut(&self) -> &'static mut TrapContext {
        self.inner
            .exclusive_access()
            .current
            .as_ref()
            .expect("no current task")
            .trap_ctx_mut()
    }

    fn set_current_task(&self, task: Arc<TaskStruct>) {
        self.inner.exclusive_access().current = Some(task);
    }

    fn take_out_current_task(&self) -> Option<Arc<TaskStruct>> {
        self.inner.exclusive_access().current.take()
    }

    fn current_task(&self) -> Option<Arc<TaskStruct>> {
        self.inner.exclusive_access().current.clone()
    }
}

pub fn cur_trap_ctx_mut() -> &'static mut TrapContext {
    PROCESSOR.trap_ctx_mut()
}

pub fn get_cur_task_arc() -> Option<Arc<TaskStruct>> {
    PROCESSOR.current_task()
}

pub fn cur_satp_token() -> usize {
    PROCESSOR.satp_token()
}

pub fn take_out_current() -> Option<Arc<TaskStruct>> {
    PROCESSOR.take_out_current_task()
}

// virtual address may be continous, but physical address may not be
pub fn translate_cur_byte_buffer_mut(buf: usize, len: usize) -> Option<Vec<&'static mut [u8]>> {
    let mut ret = Vec::new();
    let mut cur_va = VirtAddr(buf);
    let mut rem_len = len;
    while rem_len > 0 {
        let cur_slice_len = min(cur_va.floor_page().next_page().0 - cur_va.0, rem_len);
        let cur_frame = PROCESSOR
            .current_task()?
            .translate_vp(cur_va.floor_page())?;
        let slice = &mut cur_frame.get_bytes_array_mut()
            [cur_va.get_offset()..cur_va.get_offset() + cur_slice_len];
        ret.push(slice);
        cur_va.0 += cur_slice_len;
        rem_len -= cur_slice_len;
    }
    Some(ret)
}

pub fn switch_to_idle(cur_task_ctx_ptr: *mut TaskContext) {
    let idle_ctx_ptr = PROCESSOR.idle_task_ctx_ptr();
    __switch(cur_task_ctx_ptr, idle_ctx_ptr);
}

pub fn proc_schedule() {
    loop {
        if let Some(next_task) = fetch_ready_task() {
            // only idle and next task, no current handled here
            let idle_ctx_ptr = PROCESSOR.idle_task_ctx_ptr();
            let next_ctx_ptr = next_task.task_ctx_ptr();
            next_task.mark_task_status(TaskStatus::Running);
            PROCESSOR.set_current_task(next_task.clone());
            __switch(idle_ctx_ptr, next_ctx_ptr)
        } else {
            panic!("no ready task");
        }
    }
}
