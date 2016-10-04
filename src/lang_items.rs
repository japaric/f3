use core::fmt::Arguments;

#[export_name = "rust_begin_unwind"]
#[lang = "panic_fmt"]
#[linkage = "weak"]
unsafe extern "C" fn panic_fmt(msg: Arguments,
                               file: &'static str,
                               line: u32)
                               -> ! {
    iprintln!("PANIC at '{}', {}:{}", msg, file, line);

    bkpt!();

    loop {}
}
