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
//! #[export_name = "main"]
//! pub extern "C" fn main() -> ! {
//!     let x = black_box(2_f32);
//!     let y = black_box(3_f32);
//!     let z = black_box(x * y);
//!
//!     // FIXME(japaric/rustc-builtins#79) there is a problem with the
//!     // rustc-builtins (compiler-builtins-snapshot) crate and LTO. Avoid the
//!     // issue by printing these values as integers because integer formatting
//!     // doesn't need any intrinsic.
//!     iprintln!("{} * {} = {}", x as u32, y as u32, z as u32);
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
