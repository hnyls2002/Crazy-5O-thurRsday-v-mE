pub mod kernel_trap;
pub mod trap_context;

use crate::{
    config::TRAP_CTX_VIRT_ADDR,
    mm::PageTable,
    task::{exit_cur_run_next, suspend_cur_run_next, PROCESSOR},
};
use core::arch::{asm, global_asm};

use riscv::register::{scause, stval, stvec};

use crate::{config::TRAMPOLINE_VIRT_ADDR, mm::Frame, syscall_impl::syscall_dispathcer};

use self::kernel_trap::kernelvec;

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
    unsafe { stvec::write(kernelvec as usize, stvec::TrapMode::Direct) };
    let mut trap_ctx = PROCESSOR.cur_trap_ctx_mut();
    let s_cause = scause::read();
    let s_tval = stval::read();
    match s_cause.cause() {
        scause::Trap::Exception(e) => {
            match e {
                scause::Exception::UserEnvCall => {
                    // ecall is four bytes length
                    // error!("[trap_handler] sepc : {:X}", trap_ctx.s_epc);
                    // error!("[trap_handler] sp : {:X}", trap_ctx.x[2]);
                    trap_ctx.s_epc += 4;
                    let result = syscall_dispathcer(
                        trap_ctx.x[17],
                        [trap_ctx.x[10], trap_ctx.x[11], trap_ctx.x[12]],
                    ) as usize;
                    // exec will change the trap_ctx
                    trap_ctx = PROCESSOR.cur_trap_ctx_mut();
                    trap_ctx.x[10] = result;
                }
                _ => {
                    {
                        let cur_task = PROCESSOR
                            .current_arc()
                            .expect("exception handler : no current task");
                        info!(
                            "In process \"{}\", pid = {}",
                            cur_task.get_name(),
                            *cur_task.pid
                        );
                        // --------cur task drop here--------
                    }
                    info!("The exception \x1b[31m[{:?}]\x1b[34m happen at address : {:#X}, s_val : {:#X}", e,trap_ctx.s_epc, s_tval);
                    exit_cur_run_next(-1);
                    // trap_ctx.s_epc += 4;
                }
            }
        }
        scause::Trap::Interrupt(i) => match i {
            scause::Interrupt::SupervisorSoft => {
                unsafe { asm!("csrw sip, {ssip}", ssip = in(reg) !2) };
                suspend_cur_run_next();
            }
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
    let user_satp: usize = PageTable::satp_token(PROCESSOR.current_arc().unwrap().pt_entry());
    unsafe {
        // when jump back to user space, set stvec to trampoline again
        stvec::write(TRAMPOLINE_VIRT_ADDR.0, stvec::TrapMode::Direct);
        asm!("jr {addr}", 
            addr = in(reg) restore_va,
            in("a0") user_satp,
            in("a1") TRAP_CTX_VIRT_ADDR.0,
            options(noreturn));
    }
}
