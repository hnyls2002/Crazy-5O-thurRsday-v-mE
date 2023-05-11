use alloc::string::String;

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
    pub name: String,
    pub task_ctx: TaskContext,
    pub trap_ctx_frame: Frame,
    pub status: TaskStatus,
    pub user_space: MemorySet,
}

pub struct TaskStruct {
    // read only fields
    pub kernel_stack: KernelStack,
    pub pid: PIDTracker,
    inner: UPSafeCell<TaskStructInner>,
}

impl TaskStruct {
    pub fn get_name(&self) -> String {
        self.inner.exclusive_access().name.clone()
    }

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

    pub fn satp_token(&self) -> usize {
        self.inner.exclusive_access().user_space.satp_token()
    }
}

impl TaskStruct {
    pub fn new_from_elf(name: &'static str) -> Self {
        let pid = pid_alloc();
        let elf_data = load_app_by_name(name);
        let (user_space, entry_addr, user_sp) =
            MemorySet::new_from_elf(elf_data.expect("failed to load app"));
        let kernel_stack = KernelStack::new(*pid);

        // initialize the task context
        let task_ctx = TaskContext::new(kernel_stack.top_sp(), trap_return as usize);

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
            kernel_stack.top_sp(),
            trap_handler as usize,
        );

        TaskStruct {
            kernel_stack,
            pid,
            inner: UPSafeCell::new(TaskStructInner {
                name: name.into(),
                task_ctx,
                trap_ctx_frame,
                status: TaskStatus::Ready,
                user_space,
            }),
        }
    }

    // trap context : as the same as the context "when task traps in"
    // but the kernel_sp should change
    // task context : initial task context, return to trap_return
    pub fn fork_task_struct(&self) -> Self {
        let pid = pid_alloc();

        let user_space = self.inner.exclusive_access().user_space.fork_memory_set();

        let kernel_stack = KernelStack::new(*pid);

        let task_ctx = TaskContext::new(kernel_stack.top_sp(), trap_return as usize);

        let trap_ctx_frame = user_space
            .page_table
            .translate_vp(TRAP_CTX_VIRT_ADDR.floor_page())
            .unwrap();

        let trap_ctx = trap_ctx_frame.get_mut::<TrapContext>();
        // only kernel sp changes
        trap_ctx.kernel_sp = kernel_stack.top_sp();

        let inner = TaskStructInner {
            task_ctx,
            trap_ctx_frame,
            status: TaskStatus::Ready,
            name: self.get_name().clone(),
            user_space,
        };

        TaskStruct {
            kernel_stack,
            pid,
            inner: UPSafeCell::new(inner),
        }
    }

    pub fn exec_from_elf(&self, name_ptr: *const u8) -> Result<(), ()> {
        // update name
        let name_try = self
            .inner
            .exclusive_access()
            .user_space
            .page_table
            .translate_str(name_ptr);

        // if the name is not found
        let name = match name_try {
            Some(name) => name,
            None => {
                return Err(());
            }
        };

        self.inner.exclusive_access().name = name.clone();

        // pid : no change
        let elf_data = load_app_by_name(&name).expect("failed to load app");

        // kernel stack doesn't need to be updated

        // task context doesn't need to be updated

        // alloc new user_space and replace the old one
        let (user_space, entry_addr, user_sp) = MemorySet::new_from_elf(elf_data);
        self.inner.exclusive_access().user_space = user_space;

        // get new trap context frame
        let trap_ctx_frame = self
            .inner
            .exclusive_access()
            .user_space
            .page_table
            .translate_vp(TRAP_CTX_VIRT_ADDR.floor_page())
            .unwrap();
        // !!! update the trap context frame !!!
        self.inner.exclusive_access().trap_ctx_frame = trap_ctx_frame;

        // trap context : set to entry point of the new code
        let trap_ctx = trap_ctx_frame.get_mut::<TrapContext>();
        *trap_ctx = TrapContext::init_trap_ctx(
            entry_addr,
            user_sp,
            kernel_token(),
            self.kernel_stack.top_sp(),
            trap_handler as usize,
        );
        Ok(())
    }
}
