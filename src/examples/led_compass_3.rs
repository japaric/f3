// Auto-generated. Do not modify this file! Instead modify examples/led-compass-3.rs
//! A LED compass that points to the North, take 3
//!
//! ``` rust,no_run
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate f3;
//! extern crate m;
//!
//! use core::f32::consts::PI;
//!
//! use f3::I16x3;
//! use f3::led::Direction;
//! use f3::{delay, led, lsm303dlhc};
//!
//! use m::Float;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     loop {
//!         let I16x3 { x, y, .. } = lsm303dlhc::magnetic_field();
//!
//!         let theta = (y as f32).atan2(x as f32);
//!
//!         let dir = if theta < -7. * PI / 8. {
//!             Direction::North
//!         } else if theta < -5. * PI / 8. {
//!             Direction::NorthWest
//!         } else if theta < -3. * PI / 8. {
//!             Direction::West
//!         } else if theta < -PI / 8. {
//!             Direction::SouthWest
//!         } else if theta < PI / 8. {
//!             Direction::South
//!         } else if theta < 3. * PI / 8. {
//!             Direction::SouthEast
//!         } else if theta < 5. * PI / 8. {
//!             Direction::East
//!         } else if theta < 7. * PI / 8. {
//!             Direction::NorthEast
//!         } else {
//!             Direction::North
//!         };
//!
//!         led::all_off();
//!         dir.on();
//!
//!         delay::ms(100);
//!     }
//! }
//! ```
