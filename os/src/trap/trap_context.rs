/// Aligned in C style
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub sstatus: usize,
    pub sepc: usize,
    // for back to user space
    pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
}
