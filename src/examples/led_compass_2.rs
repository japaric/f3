// Auto-generated. Do not modify this file! Instead modify examples/led-compass-2.rs
//! A LED compass that points to the North, take 2
//!
//! ``` rust,no_run
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate f3;
//!
//! use f3::I16x3;
//! use f3::led::Direction;
//! use f3::{delay, led, lsm303dlhc};
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     loop {
//!         let I16x3 { x, y, .. } = lsm303dlhc::magnetic_field();
//!
//!         led::all_off();
//!
//!         // NOTE
//!         // - tan(PI / 8) ~= 2 / 5
//!         // - tan(3 * PI / 8) ~= 12 / 5
//!         let _2_x = 2 * x;
//!         let _2_y = 2 * y;
//!         let _5_x = 5 * x;
//!         let _5_y = 5 * y;
//!         let _12_x = 12 * x;
//!         let _12_y = 12 * y;
//!
//!         let dir = match (x > 0, y > 0) {
//!             (false, false) => {
//!                 if _5_y > _2_x {
//!                     Direction::North
//!                 } else if _5_y > _12_x {
//!                     Direction::NorthWest
//!                 } else {
//!                     Direction::West
//!                 }
//!             }
//!             (false, true) => {
//!                 if _5_y < -_2_x {
//!                     Direction::North
//!                 } else if _5_y < -_12_x {
//!                     Direction::NorthEast
//!                 } else {
//!                     Direction::East
//!                 }
//!             }
//!             (true, false) => {
//!                 if -_5_y < _2_x {
//!                     Direction::South
//!                 } else if _5_y < _12_x {
//!                     Direction::SouthWest
//!                 } else {
//!                     Direction::West
//!                 }
//!             }
//!             (true, true) => {
//!                 if _5_y < _2_x {
//!                     Direction::South
//!                 } else if _5_y < _12_x {
//!                     Direction::SouthEast
//!                 } else {
//!                     Direction::East
//!                 }
//!             }
//!         };
//!
//!         dir.on();
//!
//!         delay::ms(100);
//!     }
//! }
//! ```
