#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if cfg!(feature = "Error") {
            $crate::console::print(format_args!(concat!(
                "\x1b[31m", "[OS-error] ", $fmt, "\x1b[0m", "\n"
            ) $(, $($arg)+)?));
        }
    }
}

#[macro_export]
macro_rules! warn {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if cfg!(feature = "Warn") {
            $crate::console::print(format_args!(concat!(
                "\x1b[93m", "[OS-warn] ", $fmt, "\x1b[0m", "\n"
            ) $(, $($arg)+)?));
        }
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if cfg!(feature = "Info") {
            $crate::console::print(format_args!(concat!(
                "\x1b[34m", "[KFC-OS] ", $fmt, "\x1b[0m", "\n"
            ) $(, $($arg)+)?));
        }
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if cfg!(feature = "Debug") {
            $crate::console::print(format_args!(concat!(
                "\x1b[32m", "[OS-debug] ", $fmt, "\x1b[0m", "\n"
            ) $(, $($arg)+)?));
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        if cfg!(feature = "Trace") {
            $crate::console::print(format_args!(concat!(
                "\x1b[35m", "[OS-trace] ", $fmt, "\x1b[0m", "\n"
            ) $(, $($arg)+)?));
        }
    }
}
