use alloc::{
    string::String,
    sync::{Arc, Weak},
    vec::Vec,
};

use crate::{
    app_loader::load_app_by_name,
    config::TRAP_CTX_VIRT_ADDR,
    kfc_util::up_safe_cell::UPSafeCell,
    mm::{memory_set::MemorySet, Frame, PageTable, KERNEL_SPACE},
    task::pid_allocator::pid_alloc,
    trap::{trap_context::TrapContext, trap_handler, trap_return},
};

use super::{
    kernel_stack::KernelStack, pid_allocator::PIDTracker, task_context::TaskContext, INIT_PROC,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Zombie,
}

pub struct TaskStructInner {
    pub name: String,
    pub task_ctx: TaskContext,
    pub trap_ctx_frame: Frame,
    pub status: TaskStatus,
    pub user_space: MemorySet,
    pub exit_code: isize,
    pub parent: Option<Weak<TaskStruct>>,
    pub children: Vec<Arc<TaskStruct>>,
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

    pub fn pt_entry(&self) -> Frame {
        self.inner.exclusive_access().user_space.page_table.entry
    }

    pub fn task_status(&self) -> TaskStatus {
        self.inner.exclusive_access().status
    }

    pub fn mark_task_status(&self, status: TaskStatus) {
        self.inner.exclusive_access().status = status;
    }

    pub fn add_child(&self, child: Arc<TaskStruct>) {
        self.inner.exclusive_access().children.push(child);
    }

    pub fn set_parent(&self, parent: Weak<TaskStruct>) {
        self.inner.exclusive_access().parent = Some(parent);
    }

    pub fn trap_ctx_mut(&self) -> &'static mut TrapContext {
        self.inner.exclusive_access().trap_ctx_frame.get_mut()
    }

    pub fn task_ctx_ptr(&self) -> *mut TaskContext {
        &self.inner.exclusive_access().task_ctx as *const _ as *mut _
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
            PageTable::satp_token(KERNEL_SPACE.pt_entry()),
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
                exit_code: 0,
                parent: None,
                children: Vec::new(),
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
            exit_code: 0,
            parent: None,
            children: Vec::new(),
        };

        TaskStruct {
            kernel_stack,
            pid,
            inner: UPSafeCell::new(inner),
        }
    }

    pub fn exec_from_elf(&self, name_ptr: *const u8) -> Option<isize> {
        // update name
        let name = self
            .inner
            .exclusive_access()
            .user_space
            .page_table
            .translate_str(name_ptr)?;

        self.inner.exclusive_access().name = name.clone();

        // pid : no change
        let elf_data = load_app_by_name(&name)?;

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
            PageTable::satp_token(KERNEL_SPACE.pt_entry()),
            self.kernel_stack.top_sp(),
            trap_handler as usize,
        );

        Some(0)
    }
}

// system call related functions
impl TaskStruct {
    // exit the task
    pub fn exit_task(&self, exit_code: isize) {
        let mut inner = self.inner.exclusive_access();

        // change the status
        inner.status = TaskStatus::Zombie;

        // store the exit code
        inner.exit_code = exit_code;

        // free the resources
        inner.user_space.free_resources();

        // move a the child process to INIT_PROC
        for child in inner.children.iter() {
            child.set_parent(Arc::downgrade(&INIT_PROC));
            INIT_PROC.add_child(child.clone());
        }

        inner.children.clear();
    }

    pub fn wait_task(&self, pid: isize, exit_code_mut: &mut isize) -> isize {
        let mut inner = self.inner.exclusive_access();

        // no required pid found
        if inner
            .children
            .iter()
            .find(|&ch| pid == -1 || *ch.pid == pid as usize)
            .is_none()
        {
            return -1;
        }

        let zombie = inner.children.iter().position(|ch| {
            (pid == -1 || *ch.pid == pid as usize) && ch.task_status() == TaskStatus::Zombie
        });

        if let Some(zom_idx) = zombie {
            let ch = inner.children.remove(zom_idx);
            let pid = *ch.pid;

            assert!(
                Arc::strong_count(&ch) == 1,
                "zombie process has more than one reference"
            );
            *exit_code_mut = ch.inner.exclusive_access().exit_code;

            // ----------------- ch dropped here -----------------

            pid as isize
        } else {
            // still running
            return -2;
        }
    }
}
