use alloc::collections::BTreeMap;

use crate::{
    app_loader::load_app,
    config::{kernel_stack_range, TRAP_CTX_VIRT_ADDR},
    mm::{
        memory_set::{MemorySet, KERNEL_SATP},
        Frame, MapArea, MapPerm, MapType, VPRange, KERNEL_SPACE,
    },
    trap::trap_context::TrapContext,
};

use super::task_context::TaskContext;

pub struct TaskInfo {
    pub task_id: usize,
    pub addr_space: MemorySet,
    pub task_ctx: TaskContext,
    pub trap_ctx_frame: Frame,
}

impl TaskInfo {
    pub fn new_init(task_id: usize) -> Self {
        // build memory set
        let elf_data = load_app(task_id);
        let (user_space, entry_addr, user_sp) = MemorySet::new_from_elf(elf_data);

        // build the kernel stack
        let kt_range = kernel_stack_range(task_id);
        let kernel_sp = kt_range.1 .0;
        let kernel_stack = MapArea::new(
            VPRange::new(kt_range.0, kt_range.1),
            MapType::Framed(BTreeMap::new()),
            MapPerm::R | MapPerm::W,
            None,
        );
        KERNEL_SPACE
            .exclusive_access()
            .insert_new_map_area(kernel_stack);

        // initialize the task context
        // ra : return to __restore at the first time
        extern "C" {
            fn __restore_trap_ctx();
        }
        let task_ctx = TaskContext {
            kernel_sp,
            kernel_fn_ra: __restore_trap_ctx as usize,
            s_reg: [0; 12],
        };

        // initialize the trap context
        let trap_ctx_frame = user_space
            .page_table
            .translate_vp(TRAP_CTX_VIRT_ADDR.floor_page())
            .unwrap();

        let trap_ctx = trap_ctx_frame.get_mut::<TrapContext>();
        *trap_ctx = TrapContext::init_trap_ctx(
            entry_addr,
            user_sp,
            *KERNEL_SATP,
            kernel_sp,
            __restore_trap_ctx as usize,
        );

        TaskInfo {
            task_id,
            addr_space: user_space,
            task_ctx,
            trap_ctx_frame,
        }
    }
}
