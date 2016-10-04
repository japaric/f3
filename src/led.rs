//! LEDs

use peripheral;

/// All the LEDs in the "compass" starting from "North" and going clockwise.
pub static LEDS: [Led; 8] = [Led { i: 9 },
                             Led { i: 10 },
                             Led { i: 11 },
                             Led { i: 12 },
                             Led { i: 13 },
                             Led { i: 14 },
                             Led { i: 15 },
                             Led { i: 8 }];

pub struct Led {
    i: u8,
}

impl Led {
    /// Turns the LED off
    pub fn off(&self) {
        peripheral::gpioe().bsrr.write(1 << self.i + 16)
    }

    /// Turns the LED on
    pub fn on(&self) {
        peripheral::gpioe().bsrr.write(1 << self.i)
    }
}

/// Initializes the necessary stuff to drive the LEDs on and off
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let gpioe = peripheral::gpioe_mut();
    let rcc = peripheral::rcc_mut();

    // RCC: Enable GPIOE
    let ahbenr = rcc.ahbenr.read();
    rcc.ahbenr.write({
        const IOPEEN: u32 = 1 << 21;

        ahbenr | IOPEEN
    });

    // GPIOE: Configure pins 8-15 as outputs
    let moder = gpioe.moder.read();
    gpioe.moder.write({
        moder & !0xFFFF_0000 | 0x5555_0000
    });
}
