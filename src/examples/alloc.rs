// Auto-generated. Do not modify this file! Instead modify examples/alloc.rs
//! I can totally allocate stuff
//!
//! ``` rust,no_run
//! #![feature(collections)]
//! #![no_main]
//! #![no_std]
//!
//! extern crate collections;
//! #[macro_use]
//! extern crate f3;
//!
//! use collections::Vec;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     let mut xs = Vec::new();
//!     xs.push(1);
//!     xs.push(2);
//!     xs.push(3);
//!
//!     iprintln!("{:?}", xs);
//!
//!     loop {}
//! }
//! ```
