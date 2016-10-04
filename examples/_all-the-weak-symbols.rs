//! Uninteresting example that produces all the weak symbols that `f3` exposes

#![no_main]
#![no_std]

extern crate f3;

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    panic!()
}
