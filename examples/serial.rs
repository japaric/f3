//! Send a message over the "Serial Port"

#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

#[no_mangle]
pub fn main() -> ! {
    uprintln!("The quick brown fox jumps over the lazy dog.");

    loop {}
}
