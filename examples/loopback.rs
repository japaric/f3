//! Serial interface loopback
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate f3;

use f3::serial::Serial;
use rtfm::{app, Threshold};

// CONFIGURATION
const BAUD_RATE: u32 = 115_200; // bits per second

// TASKS & RESOURCES
app! {
    device: f3::stm32f30x,

    tasks: {
        USART1_EXTI25: {
            path: loopback,
            resources: [USART1],
        },
    }
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals) {
    let serial = Serial(&p.USART1);

    serial.init(&p.GPIOA, &p.RCC, BAUD_RATE);
}

// IDLE LOOP
fn idle() -> ! {
    // Sleep
    loop {
        rtfm::wfi();
    }
}

// TASKS
// Send back the received byte
fn loopback(_t: &mut Threshold, r: USART1_EXTI25::Resources) {
    let serial = Serial(&r.USART1);

    let byte = serial.read().unwrap();
    serial.write(byte).unwrap();
}
