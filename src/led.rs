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
        use peripheral::gpio::BsrrW;
        let bsrr = &peripheral::gpioe().bsrr;
        match self.i {
            8 => bsrr.write(*BsrrW::reset_value().br8(true)),
            9 => bsrr.write(*BsrrW::reset_value().br9(true)),
            10 => bsrr.write(*BsrrW::reset_value().br10(true)),
            11 => bsrr.write(*BsrrW::reset_value().br11(true)),
            12 => bsrr.write(*BsrrW::reset_value().br12(true)),
            13 => bsrr.write(*BsrrW::reset_value().br13(true)),
            14 => bsrr.write(*BsrrW::reset_value().br14(true)),
            15 => bsrr.write(*BsrrW::reset_value().br15(true)),
            _ => {}
        }
    }

    /// Turns the LED on
    pub fn on(&self) {
        use peripheral::gpio::BsrrW;
        let bsrr = &peripheral::gpioe().bsrr;
        match self.i {
            8 => bsrr.write(*BsrrW::reset_value().bs8(true)),
            9 => bsrr.write(*BsrrW::reset_value().bs9(true)),
            10 => bsrr.write(*BsrrW::reset_value().bs10(true)),
            11 => bsrr.write(*BsrrW::reset_value().bs11(true)),
            12 => bsrr.write(*BsrrW::reset_value().bs12(true)),
            13 => bsrr.write(*BsrrW::reset_value().bs13(true)),
            14 => bsrr.write(*BsrrW::reset_value().bs14(true)),
            15 => bsrr.write(*BsrrW::reset_value().bs15(true)),
            _ => {}
        }
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
    rcc.ahbenr.modify(|r| r.iopeen(true));

    // GPIOE: Configure pins 8-15 as outputs
    gpioe.moder.modify(|r| {
        r.moder8(0b01)
            .moder9(0b01)
            .moder10(0b01)
            .moder11(0b01)
            .moder12(0b01)
            .moder13(0b01)
            .moder14(0b01)
            .moder15(0b01)
    });
}
