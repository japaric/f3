#[cfg(feature = "default-panic-fmt")]
use core::fmt::Arguments;

#[cfg(feature = "default-panic-fmt")]
#[lang = "panic_fmt"]
unsafe extern "C" fn panic_fmt(msg: Arguments,
                               file: &'static str,
                               line: u32)
                               -> ! {
    iprintln!("PANIC at '{}', {}:{}", msg, file, line);

    bkpt!();

    loop {}
}
