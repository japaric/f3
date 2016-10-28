#![no_main]
#![no_std]

#[macro_use]
extern crate f3;
extern crate futuro;

use futuro::prelude::*;
use f3::i2c::{Address, I2c};

#[no_mangle]
pub fn main() -> ! {
    const MAGNETOMETER: u8 = 0b001_1110;
    const MR_REG_M: u8 = 0x02;
    const OUT_X_H_M: u8 = 0x3;

    let i2c = I2c::new().unwrap();

    // NOTE(write_all) configure the magnetometer to operate in continuous mode
    // NOTE(write+read_exact) read the magnetic field
    let buffer = i2c.write_all(Address::u7(MAGNETOMETER), &[MR_REG_M, 0b00])
        .and_then(|i2c| i2c.write(None, OUT_X_H_M))
        .and_then(|i2c| i2c.read_exact(None, [0u8; 6]))
        .and_then(|(i2c, buffer)| i2c.stop().map(move |_| buffer))
        .wait();

    iprintln!("{:?}", buffer);

    loop {
    }
}
