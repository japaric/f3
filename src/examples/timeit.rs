// Auto-generated. Do not modify this file! Instead modify examples/timeit.rs
//! Timing sections of code
//!
//! ``` rust,no_run
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate f3;
//!
//! use f3::timeit::FREQUENCY;
//! use f3::{delay, timeit};
//!
//! macro_rules! timeit {
//!     ($e:expr) => {
//!         let bias = timeit::timeit(|| {});
//!         let ticks = timeit::timeit(|| $e) - bias;
//!         iprintln!("{} took {} ticks ({} s)",
//!                   stringify!($e),
//!                   ticks,
//!                   ticks as f32 / FREQUENCY as f32);
//!     }
//! }
//!
//! #[export_name = "main"]
//! pub extern "C" fn main() -> ! {
//!     timeit!(delay::ms(1_000));
//!     timeit!(iprintln!("Hello, world!"));
//!     timeit!(iprintln!("{}", 42));
//!     timeit!(iprintln!("{}", core::f32::consts::PI));
//!
//!     loop {}
//! }
//! ```
