//! Timing a section of code

#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use f3::timeit::FREQUENCY;
use f3::{delay, timeit};

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    let ticks = timeit::timeit(|| delay::ms(1_000));

    iprintln!("{} ticks ({} s) elapsed",
              ticks,
              ticks as f32 / FREQUENCY as f32);

    loop {}
}
