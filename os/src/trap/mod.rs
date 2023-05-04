mod trap_context;

use core::arch::{asm, global_asm};

use lazy_static::lazy_static;

use crate::mm::Frame;

global_asm!(include_str!("trampoline.S"));

fn get_trampoline_frame() -> Frame {
    extern "C" {
        fn strampoline();
    }
    Frame(strampoline as usize)
}

lazy_static! {
    pub static ref TRAMPOLINE_FRAME: Frame = get_trampoline_frame();
}

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
