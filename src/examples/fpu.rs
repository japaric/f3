// Auto-generated. Do not modify this file! Instead modify examples/fpu.rs
//! You can do fast, single precision, floating point math as well
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
//!     let x = black_box(2.5_f32);
//!     let y = black_box(3.5_f32);
//!     let z = black_box(x * y);
//!
//!     iprintln!("{} * {} = {}", x, y, z);
//!
//!     loop {}
//! }
//!
//! // Magic to prevent optimizations like "constant folding" which is when LLVM
//! // performs arithmetic at compile time. This is a copy of `test::black_box`.
//! fn black_box<T>(x: T) -> T {
//!     unsafe {
//!         asm!("" :: "r"(&x));
//!     }
//!
//!     x
//! }
//! ```
