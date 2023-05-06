pub mod trap_context;

use core::arch::{asm, global_asm};

use riscv::register::{scause, stval, stvec};

use crate::{
    config::TRAMPOLINE_VIRT_ADDR, mm::Frame, syscall_impl::syscall_dispathcer,
    task::task_manager::get_cur_trap_ctx_mut,
};

global_asm!(include_str!("trampoline.S"));

pub fn trampoline_frame() -> Frame {
    extern "C" {
        fn strampoline();
    }
    Frame(strampoline as usize)
}

pub fn trap_handler() -> ! {
    // when in S-mode, disable the exception
    // the interrupt has been disabled by hardware (sstatus.sie = 0)
    unsafe { stvec::write(kernel_trap as usize, stvec::TrapMode::Direct) };
    let trap_ctx = get_cur_trap_ctx_mut();
    let s_cause = scause::read();
    let s_tval = stval::read();
    match s_cause.cause() {
        scause::Trap::Exception(e) => match e {
            scause::Exception::UserEnvCall => {
                // ecall is four bytes length
                trap_ctx.s_epc += 4;
                trap_ctx.x[10] = syscall_dispathcer(
                    trap_ctx.x[17],
                    [trap_ctx.x[10], trap_ctx.x[11], trap_ctx.x[12]],
                ) as usize;
            }
            _ => {
                info!("The exception \x1b[31m[{:?}]\x1b[34m happen", e);
                info!(
                    "at address : \x1b[31m{:X}\x1b[34m, s_val : \x1b[31m{:X}\x1b[34m",
                    trap_ctx.s_epc, s_tval
                );
            }
        },
        scause::Trap::Interrupt(i) => match i {
            scause::Interrupt::SupervisorTimer => todo!(),
            _ => panic!("{:?} is not supported!", i),
        },
    };
    trap_return()
}

/// `trap_return()` should pass the `user_satp` and `trap_ctx` to `__restore_ctx`
pub fn trap_return() -> ! {
    extern "C" {
        fn __save_trap_ctx();
        fn __restore_trap_ctx();
    }
    let restore_va =
        TRAMPOLINE_VIRT_ADDR.0 + __restore_trap_ctx as usize - __save_trap_ctx as usize;
    unsafe {
        asm!("jr {addr}", addr = in(reg) restore_va, options(noreturn));
    }
}

pub fn kernel_trap() {
    unsafe {
        asm!(".align 2");
    }
    panic!("Trap when in S-mode!");
}

pub fn machine_trap_panic() {
    unsafe {
        asm!(".align 2");
    }
    panic!("M-mode trap handler not implemented!");
}
