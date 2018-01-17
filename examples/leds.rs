//! Turns all the user LEDs on
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]

extern crate f3;

use f3::hal::stm32f30x;
use f3::hal::prelude::*;
use f3::led::Leds;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let gpioe = p.GPIOE.split(&mut rcc.ahb);

    let mut leds = Leds::new(gpioe);

    for led in leds.iter_mut() {
        led.on();
    }
}
