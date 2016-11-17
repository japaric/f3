// Auto-generated. Do not modify this file! Instead modify examples/led-roulette.rs
//! A LED roulette!
//!
//! ``` rust,no_run
//! #![no_main]
//! #![no_std]
//!
//! extern crate f3;
//!
//! use core::iter;
//!
//! use f3::led::LEDS;
//! use f3::delay;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     loop {
//!         for (current, next) in LEDS.iter()
//!             .zip(LEDS.iter().skip(1).chain(iter::once(&LEDS[0]))) {
//!             next.on();
//!             delay::ms(10);
//!             current.off();
//!             delay::ms(90);
//!         }
//!     }
//! }
//! ```
