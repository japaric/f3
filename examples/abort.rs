//! `intrinsics::abort` triggers an exception

#![feature(core_intrinsics)]
#![no_main]
#![no_std]

extern crate f3;

use core::intrinsics;

#[export_name = "main"]
pub fn main() -> ! {
    unsafe { intrinsics::abort() }
}
