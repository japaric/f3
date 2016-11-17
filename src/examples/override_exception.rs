// Auto-generated. Do not modify this file! Instead modify examples/override-exception.rs
//! Override the hard fault (or any other) exception handler
//!
//! ``` rust,no_run
//! #![feature(asm)]
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate f3;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     let _hard_fault_exception = unsafe {
//!         // After you hit this exception ...
//!         *((0x4000_0000 + 40 * 1024) as *const u32)
//!     };
//!
//!     loop {}
//! }
//!
//! #[export_name = "_hard_fault"]  // <-- Important! Note the underscore.
//! pub extern "C" fn my_hard_fault_exception_handler() {
//!     unsafe {
//!         // .. you'll reach THIS breakpoint!
//!         bkpt!();
//!     }
//!
//!     loop {}
//! }
//! ```
