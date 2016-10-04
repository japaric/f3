// Auto-generated. Do not modify this file! Instead modify examples/override-init.rs
//! Override the initialization routine that runs before `main`
//!
//! ``` rust,no_run
//! #![feature(asm)]
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate cortex_m;
//! extern crate f3;
//!
//! #[export_name = "_init"]
//! pub extern "C" fn init() {
//!     unsafe {
//!         // You'll hit this breakpoint first and ...
//!         bkpt!();
//!     }
//! }
//!
//! #[export_name = "main"]
//! pub extern "C" fn main() -> ! {
//!     unsafe {
//!         // ... then you'll reach this breakpoint.
//!         bkpt!();
//!     }
//!
//!     loop {}
//! }
//! ```
