use alloc::{sync::Arc, vec::Vec};

use crate::{kfc_util::up_safe_cell::UPSafeCell, mm::PageTable, trap::trap_context::TrapContext};

use super::{
    switch::__switch,
    task_context::TaskContext,
    task_struct::{TaskStatus, TaskStruct},
    TASK_MANAGER,
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

pub static PROCESSOR: Processor = Processor {
    inner: UPSafeCell::new(ProcessorInner {
        current: None,
        idle_task_ctx: TaskContext::empty(),
    }),
};

impl Processor {
    fn idle_task_ctx_ptr(&self) -> *mut TaskContext {
        self.inner.exclusive_access().idle_task_ctx_ptr()
    }

    pub fn cur_trap_ctx_mut(&self) -> &'static mut TrapContext {
        self.inner
            .exclusive_access()
            .current
            .as_ref()
            .expect("no current task")
            .trap_ctx_mut()
    }

    pub fn set_current(&self, task: Arc<TaskStruct>) {
        self.inner.exclusive_access().current = Some(task);
    }

    pub fn take_out_current(&self) -> Option<Arc<TaskStruct>> {
        self.inner.exclusive_access().current.take()
    }

    /// copy current task arc
    pub fn current_arc(&self) -> Option<Arc<TaskStruct>> {
        self.inner.exclusive_access().current.clone()
    }

    // virtual address may be continous, but physical address may not be
    pub fn translate_cur_byte_buffer_mut(
        &self,
        buf: usize,
        len: usize,
    ) -> Option<Vec<&'static mut [u8]>> {
        let light_pt = PageTable {
            entry: self.inner.exclusive_access().current.as_ref()?.pt_entry(),
            pt_frames: Vec::new(),
        };
        light_pt.translate_byte_buffer_mut(buf, len)
    }
}

pub fn switch_to_idle(cur_task_ctx_ptr: *mut TaskContext) {
    let idle_ctx_ptr = PROCESSOR.idle_task_ctx_ptr();
    __switch(cur_task_ctx_ptr, idle_ctx_ptr);
}

// the idle control flow
pub fn proc_schedule() {
    loop {
        if let Some(next_task) = TASK_MANAGER.fetch_ready_task() {
            // only idle and next task, no current handled here
            let idle_ctx_ptr = PROCESSOR.idle_task_ctx_ptr();
            let next_ctx_ptr = next_task.task_ctx_ptr();
            next_task.mark_task_status(TaskStatus::Running);
            PROCESSOR.set_current(next_task.clone());
            __switch(idle_ctx_ptr, next_ctx_ptr)
        } else {
            panic!("no ready task");
        }
    }
}
