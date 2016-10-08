//! Override the default exception handler

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate f3;

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    // After you hit this exception ...
    let _exception = unsafe { *((0x4000_0000 + 40 * 1024) as *const u32) };

    loop {}
}

#[export_name = "_default_exception_handler"]  // <-- Important! Note the underscore.
pub extern "C" fn my_exception_handler() {
    unsafe {
        // .. you'll reach THIS breakpoint!
        bkpt!();
    }

    loop {}
}
