#![no_main]
#![no_std]

#[macro_use]
extern crate f3;
extern crate futuro;

use f3::i2c::I2c;
use f3::l3gd20::L3gd20;
use f3::lsm303dlhc::Lsm303dlhc;
use f3::spi::Spi;
use f3::time::Instant;
use f3::timer::Timer;
use futuro::prelude::*;

#[no_mangle]
pub fn main() -> ! {
    let mut lsm303dlhdc = I2c::new().map(|i2c| Lsm303dlhc::new(i2c)).unwrap();
    let mut l3gd20 = Spi::new().map(|spi| L3gd20::new(spi)).unwrap();
    let mut timer = Timer::new().unwrap();
    let mut periodic = timer.periodic(1_000).wait();

    loop {
        let instant = Instant::now();
        let (x, y, a, ar, mf) = lsm303dlhdc.acceleration()
            .and_then(|(x, a)| {
                x.magnetic_field().map(move |(x, mf)| (x, a, mf))
            })
            .join(l3gd20.angular_rate())
            .map(|((x, a, mf), (y, ar))| (x, y, a, ar, mf))
            .wait();
        l3gd20 = y;
        lsm303dlhdc = x;
        let elapsed = instant.elapsed();

        iprintln!("{} ticks", elapsed);
        iprintln!("{:?}", (a.x(), ar.x(), mf.x()));

        periodic.next();
    }
}
