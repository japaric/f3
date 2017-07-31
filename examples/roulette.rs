//! A LED roulette!
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cast;
extern crate f3;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;

use cast::{usize, u8};
use cortex_m::peripheral::SystClkSource;
use f3::led::{self, LEDS};
use rtfm::{app, Threshold};

// CONFIGURATION
const FREQUENCY: u32 = 4;

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,

    resources: {
        static STATE: u8 = 0;
    },

    tasks: {
        SYS_TICK: {
            path: roulette,
            resources: [STATE],
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
fn roulette(_t: &mut Threshold, r: SYS_TICK::Resources) {
    let curr = **r.STATE;
    let next = (curr + 1) % u8(LEDS.len()).unwrap();

    LEDS[usize(curr)].off();
    LEDS[usize(next)].on();

    **r.STATE = next;
}
