//! Find the orientation of the board by integrating the angular rate

#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use f3::{delay, l3gd20};

#[no_mangle]
pub fn main() -> ! {
    const GAIN: f32 = 8.75e-3;
    const PERIOD: u16 = 10;
    const MS: f32 = 1e-3;

    let mut theta_z = 0.;
    loop {
        let wz = l3gd20::angular_rate().z as f32 * GAIN;
        theta_z += wz * PERIOD as f32 * MS;

        iprintln!("{}", theta_z);

        delay::ms(PERIOD);
    }
}
