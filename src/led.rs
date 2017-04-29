//! User LEDs

use stm32f30x::{GPIOE, Gpioe, Rcc};

/// All the user LEDs
pub static LEDS: [Led; 8] = [
    Led { i: 9 },
    Led { i: 10 },
    Led { i: 11 },
    Led { i: 12 },
    Led { i: 13 },
    Led { i: 14 },
    Led { i: 15 },
    Led { i: 8 },
];

/// An LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns off the LED
    pub fn off(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOE.get()).bsrr.write(|w| w.bits(1 << (self.i + 16))) }
    }

    /// Turns on the LED
    pub fn on(&self) {
        // NOTE(safe) atomic write
        unsafe { (*GPIOE.get()).bsrr.write(|w| w.bits(1 << self.i)) }
    }
}

/// Initializes all the user LEDs
pub fn init(gpioe: &Gpioe, rcc: &Rcc) {
    // Power up peripherals
    rcc.ahbenr.modify(|_, w| w.iopeen().enabled());

    // Configure pins 8-15 as outputs
    gpioe
        .moder
        .modify(
            |_, w| {
                w.moder8()
                    .output()
                    .moder9()
                    .output()
                    .moder10()
                    .output()
                    .moder11()
                    .output()
                    .moder12()
                    .output()
                    .moder13()
                    .output()
                    .moder14()
                    .output()
                    .moder15()
                    .output()
            },
        );
}
