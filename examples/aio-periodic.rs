#![no_main]
#![no_std]

extern crate f3;
extern crate futuro;

use f3::led::LEDS;
use f3::timer::Timer;
use futuro::prelude::*;

#[no_mangle]
pub fn main() -> ! {
    let mut timer = Timer::new().unwrap();

    let mut on = true;
    let mut delay = timer.periodic(100).wait();
    loop {
        if on {
            LEDS[0].on();
        } else {
            LEDS[0].off();
        }

        on = !on;

        delay.next();
    }
}
