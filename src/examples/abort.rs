// Auto-generated. Do not modify this file! Instead modify examples/abort.rs
//! `intrinsics::abort` triggers an exception
//!
//! ``` rust,no_run
//! #![feature(core_intrinsics)]
//! #![no_main]
//! #![no_std]
//!
//! extern crate f3;
//!
//! use core::intrinsics;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     unsafe { intrinsics::abort() }
//! }
//! ```
