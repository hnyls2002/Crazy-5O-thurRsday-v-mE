use core::arch::asm;

use riscv::register::{mie, mscratch, mstatus, mtvec};

// clock configuration
const CLOCK_FREQ: usize = 1250_0000;
const MSEC_PER_SEC: usize = 1000;
const TICKS_PER_SEC: usize = 100; // 10ms per tick
const INTERVAL: usize = CLOCK_FREQ / TICKS_PER_SEC;

// clint : core local interruptor
const CLINT: usize = 0x2000000;
const CLINT_MTIMECMP: usize = CLINT + 0x4000;
const CLINT_MTIME: usize = CLINT + 0xBFF8;

static mut TIMER_SCRATCH: [usize; 5] = [0; 5];

pub fn get_time() -> usize {
    unsafe { *(CLINT_MTIME as *const usize) }
}

pub fn get_time_cmp() -> usize {
    unsafe { *(CLINT_MTIMECMP as *const usize) }
}

pub fn timer_init() {
    unsafe {
        // save timer_scratch pointer
        mscratch::write(&TIMER_SCRATCH as *const _ as usize);
        // set initial trigger
        *(CLINT_MTIMECMP as *mut usize) = *(CLINT_MTIME as *const usize) + INTERVAL;
        // set mtvec
        mtvec::write(mtimer as usize, mtvec::TrapMode::Direct);
        // enable m-mode interrupts
        mstatus::set_mie();
        // enable timer interrupt
        mie::set_mtimer();
    }
}

#[naked]
#[no_mangle]
pub extern "C" fn mtimer() {
    unsafe {
        asm!(r#"
        .align 2
        
        # store the registers
        csrrw sp, mscratch, sp # now sp -> timer_scrath
        sd a0, 0(sp)
        sd a1, 8(sp)
        sd a2, 16(sp)

        # set next time
        li a0, {mtimecmp}
        ld a1, 0(a0) # a1 = mtimecmp
        li a2, {interval}
        add a1, a1, a2
        sd a1, 0(a0) # a1 = next trigger

        # delegate a supervisor-timer-interrupt
        li a0, {mip_stip}
        csrrs zero, mip, a0

        # restore
        ld a0, 0(sp)
        ld a1, 8(sp)
        ld a2, 16(sp)
        csrrw sp, mscratch, sp

        mret
        "#, 
        mtimecmp = const CLINT_MTIMECMP,
        interval = const INTERVAL,
        mip_stip = const (1<<5),
        options(noreturn))
    }
}
