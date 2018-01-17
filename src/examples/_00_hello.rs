//! Prints "Hello, world" on the OpenOCD console
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_std]
//! 
//! extern crate cortex_m_semihosting as semihosting;
//! extern crate f3;
//! 
//! use core::fmt::Write;
//! 
//! use semihosting::hio;
//! 
//! fn main() {
//!     writeln!(hio::hstdout().unwrap(), "Hello, world!").unwrap();
//! }
//! ```
// Auto-generated. Do not modify.
