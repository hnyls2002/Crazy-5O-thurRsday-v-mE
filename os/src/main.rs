#![no_main]
#![no_std]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod config;
mod kfc_logger;
mod kfc_sbi;
mod lang_items;
mod mm;

extern crate alloc;
use alloc::boxed::Box;
use core::arch::{asm, global_asm};
use riscv::register::{mepc, mideleg, mstatus, satp};

global_asm!(include_str!("entry.S"));

// global/static variables are located in .bss section
// so .bss should be cleared
fn clear_bss() {
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
        mstatus::set_mpp(mstatus::MPP::Machine);
        // jump address to kernel_main
        mepc::write(kernel_main as usize);
        // disable page
        satp::write(0);
        // delegate all interrupt and exception
        mideleg::set_sext();
        mideleg::set_ssoft();
        mideleg::set_stimer();
        // TODO : some other boot settings...
        asm!("mret");
    }
    panic!("os does not enter s-mode!");
}

#[no_mangle]
pub fn kernel_main() -> ! {
    println!("\x1b[1;31m{}\x1b[0m", kfc_sbi::LOGO);
    info!("Entering into kernel_main function!");
    info!("uart print test passed!");
    let b = Box::new(42);
    panic!("\x1b[1;33mshutdown is not implemented yet...\x1b[0m");
}
