use riscv::register::sstatus;

/// Aligned in C style
#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    pub s_status: usize,
    pub s_epc: usize,
    // for back to user space
    pub kernel_satp: usize,
    pub kernel_sp: usize,
    pub trap_handler: usize,
}

impl TrapContext {
    pub fn init_trap_ctx(
        entry: usize,
        user_sp: usize,
        kernel_satp: usize,
        kernel_sp: usize,
        trap_handler: usize,
    ) -> Self {
        let mut s_status = sstatus::read();
        s_status.set_spp(sstatus::SPP::User);
        let mut x: [usize; 32] = [0; 32];
        x[2] = user_sp;
        Self {
            x,
            s_status: s_status.bits(),
            s_epc: entry,
            kernel_satp,
            kernel_sp,
            trap_handler,
        }
    }
}
