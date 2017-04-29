//! Turns all the user LEDs on

#![feature(const_fn)]
#![feature(used)]
#![no_std]

// version = "0.2.0"
extern crate cortex_m_rt;

// version = "0.1.0"
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate f3;

use f3::led::{self, LEDS};
use f3::stm32f30x;
use rtfm::{C0, C16, P0};

// RESOURCES
peripherals!(stm32f30x, {
    GPIOE: Peripheral {
        register_block: Gpioe,
        ceiling: C0,
    },
    RCC: Peripheral {
        register_block: Rcc,
        ceiling: C0,
    },
});

// INITIALIZATION PHASE
fn init(ref prio: P0, ceil: &C16) {
    let gpioe = GPIOE.access(prio, ceil);
    let rcc = RCC.access(prio, ceil);

    led::init(&gpioe, &rcc);
}

// IDLE LOOP
fn idle(_prio: P0, _ceil: C0) -> ! {
    for led in &LEDS {
        led.on();
    }

    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
tasks!(stm32f30x, {});
