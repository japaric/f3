//! Sends Hello and then World through the first ITM stimulus port
//!
//! You'll need to run `itmdump itm.fifo` (mind the file paths) to receive the
//! message on the host. Read the [`itm`] crate documentation for details.
//!
//! [`itm`]: https://docs.rs/itm/0.1.1/itm/
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate cortex_m_rtfm as rtfm;
extern crate f3;

use rtfm::{app, Threshold};

// TASK & RESOURCES
app! {
    device: f3::stm32f30x,

    idle: {
        resources: [ITM],
    },
}

// INITIALIZATION PHASE
fn init(p: init::Peripherals) {
    iprintln!(&p.ITM.stim[0], "Hello");
}

// IDLE LOOP
fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    iprintln!(&r.ITM.stim[0], "World");

    // Sleep
    loop {
        rtfm::wfi();
    }
}
