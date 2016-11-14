// Auto-generated. Do not modify this file! Instead modify examples/override-panic-fmt.rs
//! Override `panic_fmt`
//!
//! The `default-panic-fmt` Cargo feature must be disabled for this to work.
//!
//! ``` rust,no_run
//! #![feature(asm)]
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate f3;
//!
//! use core::fmt::Arguments;
//!
//! #[export_name = "main"]
//! pub fn main() -> ! {
//!     // Panic here and ...
//!     panic!("Hello, world!")
//! }
//!
//! #[allow(dead_code)]
//! #[export_name = "rust_begin_unwind"]
//! extern "C" fn panic_fmt(_msg: Arguments, _file: &'static str, _line: u32) -> ! {
//!     unsafe {
//!         // ... you'll reach this breakpoint!
//!         bkpt!();
//!     }
//!
//!     loop {}
//! }
//! ```
