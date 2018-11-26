//! Prints "Hello, world" on the OpenOCD console
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_main]
//! #![no_std]
//! 
//! extern crate f3;
//! extern crate panic_semihosting;
//! 
//! use cortex_m_rt::entry;
//! use cortex_m_semihosting::hprintln;
//! 
//! #[entry]
//! fn main() -> ! {
//!     hprintln!("Hello, world!").unwrap();
//! 
//!     loop {}
//! }
//! ```
// Auto-generated. Do not modify.
