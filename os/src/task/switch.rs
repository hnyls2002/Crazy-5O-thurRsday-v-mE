use core::arch::asm;

#[naked]
pub fn switch(x: i32) {
    unsafe {
        asm!("", options(noreturn));
    }
}
