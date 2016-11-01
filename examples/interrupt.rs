#![feature(const_fn)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate f3;

use cortex_m::interrupt::Mutex;
use f3::led::LEDS;
use f3::peripheral;

// Shared state between the main loop and the tim7 interrupt handler
static STATE: Mutex<bool> = Mutex::new(false);

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    unsafe {
        let nvic = cortex_m::peripheral::nvic_mut();
        let rcc = peripheral::rcc_mut();
        let tim7 = peripheral::tim7_mut();

        rcc.apb1enr.modify(|_, w| w.tim7en(true));

        tim7.dier.write(|w| w.uie(true));
        tim7.psc.write(|w| w.psc(7_999));

        tim7.arr.write(|w| w.arr(100));
        tim7.egr.write(|w| w.ug(true));

        tim7.sr.read();
        tim7.sr.write(|w| w);

        nvic.iser[1].write(1 << (55 - 32));
        tim7.cr1.write(|w| w.cen(true).opm(false));
    }

    loop {
        if STATE.lock(|state| *state) {
            LEDS[0].on();
        } else {
            LEDS[0].off();
        }

        // Sleep
        unsafe { cortex_m::asm::wfi() };
    }
}

#[export_name = "_tim7"]
pub extern "C" fn tim7() {
    let tim7 = unsafe { peripheral::tim7_mut() };

    tim7.sr.read();
    tim7.sr.write(|w| w);

    STATE.lock(|state| *state = !*state);
}
