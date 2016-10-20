//! Delays
//!
//! # Implementation details
//!
//! This module uses the TIM7 peripheral and the `_tim7` interrupt under the
//! hood.

use peripheral;

/// Blocks for `n` ms
pub fn ms(n: u16) {
    unsafe {
        let tim7 = peripheral::tim7_mut();

        // The alarm (the "update event") will set off in `n` "ticks".
        // One tick = 1 ms (see `init`)
        tim7.arr.write(|w| w.arr(n));

        // Generate an update event to update the autoreload register
        tim7.egr.write(|w| w.ug(true));

        // Clear any previous "update" event by clearing the update event flag
        tim7.sr.read();
        tim7.sr.write(|w| w);

        // CEN: Enable the counter
        tim7.cr1.modify(|_, w| w.cen(true));

        // Wait until the alarm goes off (the "update event" occurs)
        while !tim7.sr.read().uif() {}

        // Clear the "update" flag
        tim7.sr.write(|w| w);
    }
}

/// Initializes the necessary stuff to be able to use delays
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let rcc = peripheral::rcc_mut();
    let tim7 = peripheral::tim7_mut();

    // RCC: Enable TIM7
    rcc.apb1enr.modify(|_, w| w.tim7en(true));

    // CEN: Disable the clock
    // OPM. Enable One Pulse Mode. Stop the counter after the next update event.
    tim7.cr1.write(|w| w.cen(false).opm(true));

    // Set pre-scaler to 8_000 -> Frequency = 1 KHz
    tim7.psc.write(|w| w.psc(7_999));
}
