//! Turns all the user LEDs on
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate f3;

use f3::led::{self, LEDS};
use rtfm::app;

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals) {
    led::init(&p.GPIOE, &p.RCC);
}

// IDLE LOOP
fn idle() -> ! {
    for led in &LEDS {
        led.on();
    }

    // Sleep
    loop {
        rtfm::wfi();
    }
}
