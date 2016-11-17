//! Timing sections of code

#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use f3::time::{FREQUENCY, Instant};
use f3::delay;

fn timeit<F>(f: F) -> u32
    where F: FnOnce()
{
    let instant = Instant::now();
    f();
    instant.elapsed()
}

macro_rules! timeit {
    ($e:expr) => {
        let bias = timeit(|| {});
        let ticks = timeit(|| $e) - bias;
        iprintln!("{} took {} ticks ({} s)",
                  stringify!($e),
                  ticks,
                  ticks as f32 / FREQUENCY as f32);
    }
}

#[no_mangle]
pub fn main() -> ! {
    timeit!(delay::ms(1_000));
    timeit!(delay::ms(1_000));
    timeit!(delay::ms(1_000));

    timeit!(iprintln!("Hello, world!"));
    timeit!(iprintln!("{}", 42));
    timeit!(iprintln!("{}", core::f32::consts::PI));

    loop {}
}
