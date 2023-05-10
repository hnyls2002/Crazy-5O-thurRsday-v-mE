use core::fmt::{Result, Write};

use crate::api::{read, write};

const FD_STDOUT: usize = 1;
const FD_STDIN: usize = 0;

pub fn getchar() -> u8 {
    let mut buf: [u8; 1] = [0; 1];
    read(FD_STDIN, &mut buf);
    buf[0]
}

struct Stdout;

// implement stdout by syscall sys_write
impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        write(FD_STDOUT, s.as_bytes());
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
