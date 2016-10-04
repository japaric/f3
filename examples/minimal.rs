//! Minimal, working example

#![no_main]
#![no_std]

extern crate f3;

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    // Your code goes here!

    loop {}
}
