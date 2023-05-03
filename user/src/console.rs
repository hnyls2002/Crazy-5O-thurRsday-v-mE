use core::fmt::{Result, Write};

use crate::sys_write;

const STDOUT: usize = 1;

struct Stdout;

// implement stdout by syscall sys_write
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        sys_write(STDOUT, s.as_bytes());
        Ok(())
    }
}

pub fn print(args: core::fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}
