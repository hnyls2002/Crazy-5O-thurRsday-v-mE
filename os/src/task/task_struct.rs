use alloc::collections::BTreeMap;

use crate::{
    app_loader::load_app,
    config::{kernel_stack_range, TRAP_CTX_VIRT_ADDR},
    mm::{
        kernel_space::{add_kernel_stack, kernel_token},
        memory_set::MemorySet,
        Frame, MapArea, MapPerm, MapType, VPRange,
    },
    trap::{trap_context::TrapContext, trap_handler, trap_return},
};

use super::task_context::TaskContext;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
    Excited,
}

pub struct TaskStruct {
    pub task_id: usize,
    pub status: TaskStatus,
    pub addr_space: MemorySet,
    pub task_ctx: TaskContext,
    pub trap_ctx_frame: Frame,
}

impl TaskStruct {
    pub fn new_init(task_id: usize) -> Self {
        // build memory set
        let elf_data = load_app(task_id);
        let (user_space, entry_addr, user_sp) =
            MemorySet::new_from_elf(elf_data.expect("failed to load app"));

        // build the kernel stack
        let kt_range = kernel_stack_range(task_id);
        let kernel_sp = (kt_range.1).0;
        let kernel_stack = MapArea::new(
            VPRange::new(kt_range.0, kt_range.1),
            MapType::Framed(BTreeMap::new()),
            MapPerm::R | MapPerm::W,
            None,
        );

        add_kernel_stack(kernel_stack);

        // initialize the task context
        let task_ctx = TaskContext::new(kernel_sp, trap_return as usize);

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
            kernel_sp,
            trap_handler as usize,
        );

        TaskStruct {
            task_id,
            status: TaskStatus::Ready,
            addr_space: user_space,
            task_ctx,
            trap_ctx_frame,
        }
    }
}
