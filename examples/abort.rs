//! `intrinsics::abort` triggers an exception

#![feature(core_intrinsics)]
#![no_main]
#![no_std]

extern crate f3;

use core::intrinsics;

#[no_mangle]
pub fn main() -> ! {
    unsafe { intrinsics::abort() }
}
