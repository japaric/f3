//! Delays
//!
//! # Implementation details
//!
//! This module uses the TIM7 peripheral and the `_tim7` interrupt under the hood.

use cortex_m::{self, asm};
use peripheral;

/// Blocks for `n` ms
pub fn ms(n: u16) {
    #[allow(dead_code)]
    #[export_name = "_tim7"]
    #[linkage = "weak"]
    pub unsafe extern "C" fn interrupt_handler() {
        // Clear the update flag
        let tim7 = peripheral::tim7_mut();
        tim7.sr.write(0);
    }

    unsafe {
        let tim7 = peripheral::tim7_mut();

        // The alarm (the "update event") will set off in `n` "ticks".
        // One tick = 1 ms (see `init`)
        tim7.arr.write(n);
        let cr1 = tim7.cr1.read();
        tim7.cr1.write({
            // Counter ENanbled
            const CEN: u16 = 1 << 0;

            cr1 | CEN
        });

        // XXX this assumes that `_tim7` is the only interrupt that can occur.
        asm::wfi();
    }
}

/// Initializes the necessary stuff to be able to use delays
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let nvic = cortex_m::peripheral::nvic_mut();
    let rcc = peripheral::rcc_mut();
    let tim7 = peripheral::tim7_mut();

    // RCC: Enable TIM7
    let apb1enr = rcc.apb1enr.read();
    rcc.apb1enr.write({
        const TIM7EN: u32 = 1 << 5;

        apb1enr | TIM7EN
    });

    let cr1 = tim7.cr1.read();
    tim7.cr1.write({
        // Disable the clock
        const CEN: u16 = 1 << 0;
        // One Pulse Mode. Stop the counter after the next update event.
        const OPM: u16 = 1 << 3;

        (cr1 | OPM) & !CEN
    });

    // TIM7: Enable "update" interrupts
    tim7.dier.write({
        const UIE: u16 = 1 << 0;

        UIE
    });

    // NVIC: Unmask the interrupt (N = 55)
    nvic.iser[1].write(1 << (55 - 32));

    // TIM7: Set pre-scaler to 8_000 -> Frequency = 1 KHz
    tim7.psc.write(7_999);
}
