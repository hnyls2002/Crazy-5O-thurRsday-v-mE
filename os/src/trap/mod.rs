use core::arch::asm;

pub fn s_mode_trap_handler() {
    unsafe {
        asm!(".align 2");
    }
    panic!("S-mode trap handler!");
}

pub fn m_mode_trap_handler() {
    unsafe {
        asm!(".align 2");
    }
    panic!("M-mode trap handler!");
}
