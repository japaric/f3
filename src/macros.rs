/// Macro for sending `print!`-formatted messages to the ITM (Instrumentation
/// Trace Macrocell).
#[macro_export]
macro_rules! iprint {
    ($s:expr) => {
        $crate::itm::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::itm::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages to the ITM, with a newline
#[macro_export]
macro_rules! iprintln {
    ($fmt:expr) => {
        iprint!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        iprint!(concat!($fmt, "\n"), $($arg)*)
    };
}

/// Macro for sending `print!`-formatted messages over the Serial Port
#[macro_export]
macro_rules! uprint {
    ($s:expr) => {
        $crate::serial::write_str($s)
    };
    ($($arg:tt)*) => {
        $crate::serial::write_fmt(format_args!($($arg)*))
    };
}

/// Macro for sending `print!`-formatted messages over the Serial Port, with a
/// newline
#[macro_export]
macro_rules! uprintln {
    ($fmt:expr) => {
        uprint!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        uprint!(concat!($fmt, "\n"), $($arg)*)
    };
}
