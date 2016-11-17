// Auto-generated. Do not modify this file! Instead modify examples/override-default-exception-handler.rs
//! Override the default exception handler
//!
//! The `default-exception-handler` Cargo feature must be disabled for this to
//! work.
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
//!     // After you hit this exception ...
//!     let _exception = unsafe { *((0x4000_0000 + 40 * 1024) as *const u32) };
//!
//!     loop {}
//! }
//!
//! #[export_name = "_default_exception_handler"]  // <-- Important! Note the underscore.
//! pub extern "C" fn my_exception_handler() {
//!     unsafe {
//!         // .. you'll reach THIS breakpoint!
//!         bkpt!();
//!     }
//!
//!     loop {}
//! }
//! ```
