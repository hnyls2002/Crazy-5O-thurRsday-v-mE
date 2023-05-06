// __switch function will exchange the kernel stack
// some information should be exchanged by *.S function
// __switch(task_ctx1, task_ctx2)
#[repr(C)]
pub struct TaskContext {
    pub kernel_sp: usize,
    pub kernel_ra: usize,
    pub s_reg: [usize; 12],
}

impl TaskContext {
    /// back to `trap_return()` function as initial `ra`
    pub fn new(kernel_sp: usize, kernel_ra: usize) -> Self {
        Self {
            kernel_sp,
            kernel_ra,
            s_reg: [0; 12],
        }
    }
}
