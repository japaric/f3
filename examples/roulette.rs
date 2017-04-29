//! A LED roulette!

#![feature(const_fn)]
#![feature(used)]
#![no_std]

// version = "0.2.0", default-features = false
extern crate cast;

// version = "0.2.0"
extern crate cortex_m_rt;

// version = "0.1.0"
#[macro_use]
extern crate cortex_m_rtfm as rtfm;

extern crate f3;

use cast::{u8, usize};
use f3::led::{self, LEDS};
use f3::stm32f30x::interrupt::Tim7;
use f3::stm32f30x;
use f3::timer::Timer;
use rtfm::{C0, C1, C16, Local, P0, P1};

// CONFIGURATION
const FREQUENCY: u32 = 4; // Hz

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
    TIM7: Peripheral {
        register_block: Tim7,
        ceiling: C1,
    },
});

// INITIALIZATION PHASE
fn init(ref prio: P0, ceil: &C16) {
    let gpioe = GPIOE.access(prio, ceil);
    let rcc = RCC.access(prio, ceil);
    let tim7 = TIM7.access(prio, ceil);
    let timer = Timer(&tim7);

    led::init(&gpioe, &rcc);
    timer.init(&rcc, FREQUENCY);
    timer.resume();
}

// IDLE LOOP
fn idle(_prio: P0, _ceil: C0) -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
tasks!(stm32f30x, {
    roulette: Task {
        interrupt: Tim7,
        priority: P1,
        enabled: true,
    },
});

fn roulette(mut task: Tim7, ref prio: P1, ref ceil: C1) {
    static STATE: Local<u8, Tim7> = Local::new(0);

    let tim7 = TIM7.access(prio, ceil);
    let timer = Timer(&tim7);

    if timer.clear_update_flag().is_ok() {
        let state = STATE.borrow_mut(&mut task);

        let curr = *state;
        let next = (curr + 1) % u8(LEDS.len()).unwrap();

        LEDS[usize(curr)].off();
        LEDS[usize(next)].on();

        *state = next;
    } else {
        // Only reachable through `rtfm::request(roulette)`
        #[cfg(debug_assertion)]
        unreachable!()
    }
}
