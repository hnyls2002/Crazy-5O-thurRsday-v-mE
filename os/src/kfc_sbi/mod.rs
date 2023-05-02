#![allow(dead_code)]
use core::arch::asm;

use crate::{error, info};

pub mod mmio;
pub mod uart;

pub const LOGO: &str = r" __  ___  _______   ______      _______..______    __  
|  |/  / |   ____| /      |    /       ||   _  \  |  | 
|  '  /  |  |__   |  ,----'   |   (----`|  |_)  | |  | 
|    <   |   __|  |  |         \   \    |   _  <  |  | 
|  .  \  |  |     |  `----..----)   |   |  |_)  | |  | 
|__|\__\ |__|      \______||_______/    |______/  |__| ";

const EID_SRST: usize = eid_from_str("SRST") as usize;
const SBI_SHUTDOWN: usize = 0;

const fn eid_from_str(name: &str) -> i32 {
    match *name.as_bytes() {
        [a] => i32::from_be_bytes([0, 0, 0, a]),
        [a, b] => i32::from_be_bytes([0, 0, a, b]),
        [a, b, c] => i32::from_be_bytes([0, a, b, c]),
        [a, b, c, d] => i32::from_be_bytes([a, b, c, d]),
        _ => panic!("Invalid EID name"),
    }
}

#[inline(always)]
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x16") fid,
            in("x17") eid,
        );
    }
    ret
}

// 0 : normal shutdown
// 1 : panic exit
pub fn sbi_shutdown(reason: usize) -> ! {
    if let 0 = reason {
        info!("Normal shutdown...")
    } else {
        error!("Panic exit the OS!")
    }
    const VIRT_TEST: *mut u32 = 0x10_0000 as *mut u32;
    const PASS: u32 = 0x5555;
    unsafe { VIRT_TEST.write_volatile(PASS) };
    panic!("sbi_shutdown failed");
}
