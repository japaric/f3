//! Delays

use peripheral;

/// Blocks for `n` ms
// TODO interrupt + sleep (`wfi`)
pub fn ms(n: u16) {
    unsafe {
        let tim6 = peripheral::tim6_mut();

        // The alarm (the "update event") will set off in `n` "ticks".
        // One tick = 1 ms (see `init`)
        tim6.arr.write(n);
        tim6.cr1.write({
            // Counter ENanbled
            const CEN: u16 = 1 << 0;
            // One Pulse Mode. Stop the counter after the next update event.
            const OPM: u16 = 1 << 3;

            OPM | CEN
        });
    }

    {
        let sr = &peripheral::tim6().sr;

        // Update Interrupt Flag. 1 = The alarm went off
        const UIF: u16 = 1 << 0;

        // Now we wait ...
        while sr.read() != UIF {}
    }

    // Clear the flag
    unsafe {
        peripheral::tim6_mut().sr.write(0);
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
    let tim6 = peripheral::tim6_mut();

    // RCC: Enable TIM6
    let apb1enr = rcc.apb1enr.read();
    rcc.apb1enr.write({
        const TIM6EN: u32 = 1 << 4;

        apb1enr | TIM6EN
    });

    // TIM6: Set pre-scaler to 8_000 -> Frequency = 1 KHz
    tim6.psc.write(7_999);
}
