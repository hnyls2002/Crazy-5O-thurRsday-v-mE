#![no_main]
#![no_std]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![feature(naked_functions, asm_const, asm_sym)]

#[macro_use]
mod console;
#[macro_use]
mod kfc_logger;

mod app_loader;
mod config;
mod kfc_sbi;
mod kfc_util;
mod lang_items;
mod mm;
mod syscall_impl;
mod task;
mod trap;

extern crate alloc;

use core::arch::{asm, global_asm};

use riscv::register::{mepc, mstatus, mtvec, satp, stvec, utvec::TrapMode};

use crate::{
    config::BOOT_STACK_SIZE,
    kfc_sbi::sbi_shutdown,
    trap::{m_mode_trap_handler, s_mode_trap_handler},
};

#[naked]
#[no_mangle]
#[link_section = ".text.entry"]
pub unsafe extern "C" fn _start() -> ! {
    #[link_section = ".bss.stack"]
    static mut BOOT_STACK: [u8; BOOT_STACK_SIZE] = [0; BOOT_STACK_SIZE];

    asm!(
    "la sp, {stack} + {stack_size}",
    "call {m_start}",
    stack = sym BOOT_STACK,
    stack_size = const BOOT_STACK_SIZE,
    m_start = sym machine_start,
    options(noreturn));
}

// global_asm!(include_str!("entry.S"));
global_asm!(include_str!("link_app.S"));

// global/static variables are located in .bss section
// so .bss should be cleared
#[no_mangle]
#[inline(never)]
pub fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|p| unsafe {
        // write_volatile : without reading or dropping the old value
        (p as *mut u8).write_volatile(0);
    })
}

#[no_mangle]
pub fn machine_start() -> ! {
    clear_bss();
    unsafe {
        // set previous mode m-mode
        mstatus::set_mpp(mstatus::MPP::Supervisor);

        // jump address to kernel_main
        mepc::write(kernel_main as usize);

        // disable page
        satp::write(0);

        // delegate all interrupt and exception
        asm!("csrw mideleg, {}", in(reg) !0);
        asm!("csrw medeleg, {}", in(reg) !0);
        asm!("csrw mcounteren, {}", in(reg) !0);

        // set sie to enable all interrupt
        asm!("csrw sie, {}", in(reg) 0x222);

        // TODO : for temporary test
        mtvec::write(m_mode_trap_handler as usize, TrapMode::Direct);

        // physical memory protection
        asm!("csrw pmpaddr0, {}", in(reg) 0x3fffffffffffff as usize);
        asm!("csrw pmpcfg0, {}", in(reg) 0xf);

        // TODO : some other boot settings...

        asm!("mret");
    }
    panic!("os does not enter s-mode!");
}

#[no_mangle]
pub fn kernel_main() -> ! {
    // TODO : temporary set S-mode trap handler
    unsafe { stvec::write(s_mode_trap_handler as usize, TrapMode::Direct) };

    info!("Entering into kernel_main function!");
    info!("UART print test passed!");
    println!("\x1b[34m{}\x1b[0m", kfc_sbi::LOGO);
    kernel_init();

    sbi_shutdown(0);
}

pub fn kernel_init() {
    mm::init();
    task::init();
}
