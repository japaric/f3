//! A LED compass that points to the North, take 1

#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use f3::I16x3;
use f3::led::Direction;
use f3::{delay, led, lsm303dlhc};

#[no_mangle]
pub fn main() -> ! {
    loop {
        let I16x3 { x, y, .. } = lsm303dlhc::magnetic_field();

        led::all_off();

        let dir = match (x > 0, y > 0) {
            (false, false) => Direction::NorthWest,
            (false, true) => Direction::NorthEast,
            (true, false) => Direction::SouthWest,
            (true, true) => Direction::SouthEast,
        };

        dir.on();

        delay::ms(100);
    }
}
