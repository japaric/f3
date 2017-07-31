//! Sharing memory using `Resource`
//!
//! This builds on top of the `concurrent` example. The `loopback` task now
//! additionally parses the received data as a command. Two commands are
//! available:
//!
//! - `reverse` - reverses the spin direction of the LED roulette
//! - `reset` - moves the roulette back to its start position (North)
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(const_fn)]
#![feature(proc_macro)]
#![no_std]

extern crate cast;
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate f3;
extern crate heapless;

use cast::{usize, u8};
use cortex_m::peripheral::SystClkSource;
use f3::led::{self, LEDS};
use f3::serial::Serial;
use heapless::Vec;
use rtfm::{app, Resource, Threshold};

// CONFIGURATION
const BAUD_RATE: u32 = 115_200; // bits per second
const FREQUENCY: u32 = 4; // Hz

// TASK & RESOURCES
app!{
    device: f3::stm32f30x,

    resources: {
        static BUFFER: Vec<u8, [u8; 16]> = Vec::new([0; 16]);
        static SHARED: State = State::new();
        static STATE: u8 = 0;
    },

    tasks: {
        USART1_EXTI25: {
            path: receive,
            priority: 1,
            resources: [BUFFER, SHARED, USART1],
        },

        SYS_TICK: {
            path: roulette,
            priority: 2,
            resources: [SHARED, STATE],
        },
    },
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals, _r: init::Resources) {
    led::init(&p.GPIOE, &p.RCC);
    Serial(&p.USART1).init(&p.GPIOA, &p.RCC, BAUD_RATE);

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
fn receive(t: &mut Threshold, mut r: USART1_EXTI25::Resources) {
    let serial = Serial(&r.USART1);

    let byte = serial.read().unwrap();
    if serial.write(byte).is_err() {
        // As we are echoing the bytes as soon as they arrive, it should
        // be impossible to have a TX buffer overrun
        #[cfg(debug_assertions)]
        unreachable!()
    }

    if byte == b'\r' {
        // end of command

        match &***r.BUFFER {
            b"bounce" => {
                r.SHARED
                    .claim_mut(t, |shared, _| { shared.mode = Mode::Bounce; });
            }
            b"continuous" => {
                r.SHARED.claim_mut(
                    t,
                    |shared, _| { shared.mode = Mode::Continuous; },
                );
            }
            b"reverse" => {
                r.SHARED.claim_mut(t, |shared, _| {
                    shared.direction = shared.direction.reverse();
                });
            }
            _ => {}
        }

        r.BUFFER.clear();
    } else {
        if r.BUFFER.push(byte).is_err() {
            // error: buffer full
            // KISS: we just clear the buffer when it gets full
            r.BUFFER.clear();
        }
    }
}

fn roulette(_t: &mut Threshold, r: SYS_TICK::Resources) {
    let curr = **r.STATE;

    let mut direction = r.SHARED.direction;

    if curr == 0 && r.SHARED.mode == Mode::Bounce {
        direction = direction.reverse();
        r.SHARED.direction = direction;
    }

    let n = u8(LEDS.len()).unwrap();
    let next = match direction {
        Direction::Clockwise => (curr + 1) % n,
        Direction::Counterclockwise => curr.checked_sub(1).unwrap_or(n - 1),
    };

    LEDS[usize(curr)].off();
    LEDS[usize(next)].on();

    **r.STATE = next;
}

// SUPPORT CODE
#[derive(Clone, Copy)]
enum Direction {
    Clockwise,
    Counterclockwise,
}

impl Direction {
    fn reverse(self) -> Self {
        match self {
            Direction::Clockwise => Direction::Counterclockwise,
            Direction::Counterclockwise => Direction::Clockwise,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    Bounce,
    Continuous,
}

pub struct State {
    direction: Direction,
    mode: Mode,
}

impl State {
    const fn new() -> Self {
        State {
            direction: Direction::Clockwise,
            mode: Mode::Continuous,
        }
    }
}
