#[repr(C)]
#[derive(Copy, Clone)]
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
    /// an empty
    pub const fn empty() -> Self {
        Self {
            kernel_sp: 0,
            kernel_ra: 0,
            s_reg: [0; 12],
        }
    }
}
