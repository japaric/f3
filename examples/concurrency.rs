//! LED roulette and serial loopback running concurrently
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate f3;

use f3::Serial;
use f3::led::{self, LEDS};
use f3::prelude::*;
use f3::serial::Event;
use f3::time::Hertz;
use cortex_m::peripheral::SystClkSource;
use cast::{usize, u8};
use rtfm::{app, Threshold};

// CONFIGURATION
const BAUD_RATE: Hertz = Hertz(115_200);
const DIVISOR: u32 = 4;


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

        USART1_EXTI25: {
            path: loopback,
            resources: [USART1],
        },
    }
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(p.GPIOE, p.RCC);

    let serial = Serial(p.USART1);
    serial.init(BAUD_RATE.invert(), Some(p.DMA1), p.GPIOA, p.RCC);
    serial.listen(Event::Rxne);

    p.SYST.set_clock_source(SystClkSource::Core);
    p.SYST.set_reload(8_000_000 / DIVISOR);
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
fn loopback(_t: &mut Threshold, r: USART1_EXTI25::Resources) {
    let serial = Serial(&**r.USART1);

    if let Ok(byte) = serial.read() {
        if serial.write(byte).is_err() {
            // As we are echoing the bytes as soon as they arrive, it should
            // be impossible to have a TX buffer overrun
            #[cfg(debug_assertions)]
            unreachable!()
        }
    } else {
        // Only reachable through `rtfm::request(loopback)`
        #[cfg(debug_assertions)]
        unreachable!()
    }
}

fn roulette(_t: &mut Threshold, r: SYS_TICK::Resources) {
    let curr = **r.STATE;
    let next = (curr + 1) % u8(LEDS.len()).unwrap();

    LEDS[usize(curr)].off();
    LEDS[usize(next)].on();

    **r.STATE = next;
}
