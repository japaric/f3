//! Override the TIM7 (or any other) interrupt handler

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use f3::delay;

#[no_mangle]
pub fn main() -> ! {
    // This function uses the TIM7 interrupt under the hood. After a second has
    // passed, the `_tim7` interrupt handler will be called and ...
    delay::ms(1_000);

    loop {}
}

#[export_name = "_tim7"]  // <-- Important! Note the underscore.
pub extern "C" fn my_tim7_interrupt_handler() {
    unsafe {
        // .. you'll reach THIS breakpoint!
        bkpt!();
    }

    loop {}
}
