use crate::{
    app_loader::load_app_by_name,
    config::TRAP_CTX_VIRT_ADDR,
    kfc_util::up_safe_cell::UPSafeCell,
    mm::{kernel_space::kernel_token, memory_set::MemorySet, Frame, Page},
    task::pid_allocator::pid_alloc,
    trap::{trap_context::TrapContext, trap_handler, trap_return},
};

use super::{kernel_stack::KernelStack, pid_allocator::PIDTracker, task_context::TaskContext};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Excited,
}

pub struct TaskStructInner {
    pub task_ctx: TaskContext,
    pub trap_ctx_frame: Frame,
    pub status: TaskStatus,
    pub user_space: MemorySet,
}

pub struct TaskStruct {
    // read only fields
    pub kernel_stack: KernelStack,
    pub pid: PIDTracker,
    pub token: usize,
    inner: UPSafeCell<TaskStructInner>,
}

impl TaskStruct {
    pub fn task_status(&self) -> TaskStatus {
        self.inner.exclusive_access().status
    }

    pub fn mark_task_status(&self, status: TaskStatus) {
        self.inner.exclusive_access().status = status;
    }

    pub fn translate_vp(&self, vp: Page) -> Option<Frame> {
        self.inner
            .exclusive_access()
            .user_space
            .page_table
            .translate_vp(vp)
    }

    pub fn trap_ctx_mut(&self) -> &'static mut TrapContext {
        self.inner.exclusive_access().trap_ctx_frame.get_mut()
    }

    pub fn task_ctx_ptr(&self) -> *mut TaskContext {
        &self.inner.exclusive_access().task_ctx as *const _ as *mut _
    }
}

impl TaskStruct {
    pub fn new_from_elf(name: &str) -> Self {
        let pid = pid_alloc();
        let elf_data = load_app_by_name(name);
        let (user_space, entry_addr, user_sp) =
            MemorySet::new_from_elf(elf_data.expect("failed to load app"));
        let kernel_stack = KernelStack::new(*pid);

        // initialize the task context
        let task_ctx = TaskContext::new(kernel_stack.sp(), trap_return as usize);

        // initialize the trap context
        let trap_ctx_frame = user_space
            .page_table
            .translate_vp(TRAP_CTX_VIRT_ADDR.floor_page())
            .unwrap();

        let trap_ctx = trap_ctx_frame.get_mut::<TrapContext>();
        *trap_ctx = TrapContext::init_trap_ctx(
            entry_addr,
            user_sp,
            kernel_token(),
            kernel_stack.sp(),
            trap_handler as usize,
        );

        TaskStruct {
            kernel_stack,
            pid,
            token: user_space.get_satp_token(),
            inner: UPSafeCell::new(TaskStructInner {
                task_ctx,
                trap_ctx_frame,
                status: TaskStatus::Ready,
                user_space,
            }),
        }
    }
}
