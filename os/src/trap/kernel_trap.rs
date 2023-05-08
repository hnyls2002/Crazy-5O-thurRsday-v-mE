use core::arch::asm;

use riscv::register::{scause, stval};

#[naked]
#[no_mangle]
pub extern "C" fn kernelvec() {
    unsafe {
        asm!(
            r#"
.altmacro
.macro SAVE_GP n
    sd x\n, \n*8(sp)
.endm
.macro RESTORE_GP n
    ld x\n, \n*8(sp)
.endm

    .align 2

    addi sp, sp, -256

    .set n, 1
    .rept 31
        SAVE_GP %n
        .set n, n+1
    .endr

    call {handler}

    .set n, 1
    .rept 31
        RESTORE_GP %n
        .set n, n+1
    .endr 

    addi sp, sp, 256

    sret
    "#,
        handler = sym kernel_trap_handler,
        options(noreturn));
    }
}

pub fn kernel_trap_handler() {
    let s_cause = scause::read();
    let s_val = stval::read();
    panic!(
        "Trap when in S-mode! [{:?}] , at address : {:#X}",
        s_cause.cause(),
        s_val
    );
}
