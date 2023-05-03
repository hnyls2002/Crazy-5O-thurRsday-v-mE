#![no_main]
#![no_std]
#![feature(panic_info_message)]

#[macro_use]
mod console;
#[macro_use]
mod kfc_logger;

mod config;
mod kfc_sbi;
mod kfc_util;
mod lang_items;
mod mm;
mod syscall_impl;
mod trap;

extern crate alloc;

use core::arch::{asm, global_asm};
use mm::{activate_kernel_space, frame_allocator_init, heap_init, heap_test, mm_test};
use riscv::register::{mepc, mstatus, mtvec, satp, stvec, utvec::TrapMode};

use crate::{
    kfc_sbi::sbi_shutdown,
    trap::{m_mode_trap_handler, s_mode_trap_handler},
};

global_asm!(include_str!("entry.S"));

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
    // buddy allocator
    heap_init();
    heap_test();

    // physical frame allocator
    frame_allocator_init();

    mm_test::remap_test();
    activate_kernel_space();

    // exception test code...
    unsafe {
        // illegal instruction
        error!("mtvec: {:#X?}", mtvec::read());

        // page fault
        let addr = 0x8090_0000 as *mut usize;
        addr.write_volatile("Hello, world!\0".as_ptr() as usize);
    };
}
