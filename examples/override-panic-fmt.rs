//! Override `panic_fmt`
//!
//! The `default-panic-fmt` Cargo feature must be disabled for this to work.

#![feature(asm)]
#![feature(lang_items)]
#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use core::fmt::Arguments;

#[no_mangle]
pub fn main() -> ! {
    // Panic here and ...
    panic!("Hello, world!")
}

#[lang = "panic_fmt"]
extern "C" fn panic_fmt(_msg: Arguments, _file: &'static str, _line: u32) -> ! {
    unsafe {
        // ... you'll reach this breakpoint!
        bkpt!();
    }

    loop {}
}
