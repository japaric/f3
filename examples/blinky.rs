//! Blinks an LED
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate f3;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use cortex_m::peripheral::SystClkSource;
use f3::led::{self, LEDS};
use rtfm::{app, Threshold};

// CONFIGURATION
const FREQUENCY: u32 = 4; // Hz

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,

    resources: {
        static ON: bool = false;
    },

    tasks: {
        SYS_TICK: {
            path: toggle,
            resources: [ON],
        },
    },
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(p.GPIOE, p.RCC);

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000 / FREQUENCY);
    p.SYST.enable_interrupt();
    p.SYST.enable_counter();
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
// Toggle the state of the LED
fn toggle(_t: &mut Threshold, r: SYS_TICK::Resources) {
    **r.ON = !**r.ON;

    if **r.ON {
        LEDS[0].on();
    } else {
        LEDS[0].off();
    }
}
