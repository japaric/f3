#[cfg(feature = "default-panic-fmt")]
use core::fmt::Arguments;

#[allow(private_no_mangle_fns)]
#[cfg(feature = "default-panic-fmt")]
#[lang = "panic_fmt"]
#[no_mangle]
unsafe extern "C" fn panic_fmt(msg: Arguments,
                               file: &'static str,
                               line: u32)
                               -> ! {
    iprintln!("PANIC at '{}', {}:{}", msg, file, line);

    bkpt!();

    loop {}
}
