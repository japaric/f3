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
        use peripheral::tim::SrW;
        peripheral::tim7_mut().sr.write(*SrW::reset_value().uif(false));
    }

    unsafe {
        let tim7 = peripheral::tim7_mut();

        // The alarm (the "update event") will set off in `n` "ticks".
        // One tick = 1 ms (see `init`)
        use peripheral::tim::ArrW;
        tim7.arr.write(*ArrW::reset_value().arr(n));

        use peripheral::tim::EgrW;
        tim7.egr.write(*EgrW::reset_value().ug(true));

        // CEN: Enable the counter
        tim7.cr1.modify(|r| r.cen(true));

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
    rcc.apb1enr.modify(|r| r.tim7en(true));

    // CEN: Disable the clock
    // OPM. Enable One Pulse Mode. Stop the counter after the next update event.
    use peripheral::tim::Cr1W;
    tim7.cr1.write(*Cr1W::reset_value().cen(false).opm(true));

    // Enable "update" interrupts
    use peripheral::tim::DierW;
    tim7.dier.write(*DierW::reset_value().uie(true));

    // NVIC: Unmask the interrupt (N = 55)
    nvic.iser[1].write(1 << (55 - 32));

    // Set pre-scaler to 8_000 -> Frequency = 1 KHz
    use peripheral::tim::PscW;
    tim7.psc.write(*PscW::reset_value().psc(7_999));
}
