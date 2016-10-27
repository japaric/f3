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

/// A single LED
pub struct Led {
    i: u8,
}

impl Led {
    /// Turns the LED off
    pub fn off(&self) {
        let bsrr = &peripheral::gpioe().bsrr;
        match self.i {
            8 => bsrr.write(|w| w.br8(true)),
            9 => bsrr.write(|w| w.br9(true)),
            10 => bsrr.write(|w| w.br10(true)),
            11 => bsrr.write(|w| w.br11(true)),
            12 => bsrr.write(|w| w.br12(true)),
            13 => bsrr.write(|w| w.br13(true)),
            14 => bsrr.write(|w| w.br14(true)),
            15 => bsrr.write(|w| w.br15(true)),
            _ => {}
        }
    }

    /// Turns the LED on
    pub fn on(&self) {
        let bsrr = &peripheral::gpioe().bsrr;
        match self.i {
            8 => bsrr.write(|w| w.bs8(true)),
            9 => bsrr.write(|w| w.bs9(true)),
            10 => bsrr.write(|w| w.bs10(true)),
            11 => bsrr.write(|w| w.bs11(true)),
            12 => bsrr.write(|w| w.bs12(true)),
            13 => bsrr.write(|w| w.bs13(true)),
            14 => bsrr.write(|w| w.bs14(true)),
            15 => bsrr.write(|w| w.bs15(true)),
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
    rcc.ahbenr.modify(|_, w| w.iopeen(true));

    // GPIOE: Configure pins 8-15 as outputs
    gpioe.moder.modify(|_, w| {
        w.moder8(0b01)
            .moder9(0b01)
            .moder10(0b01)
            .moder11(0b01)
            .moder12(0b01)
            .moder13(0b01)
            .moder14(0b01)
            .moder15(0b01)
    });
}

/// An enum over the LEDs, each LED has associated to it a direction
pub enum Direction {
    /// North
    North,
    /// Northeast
    NorthEast,
    /// East
    East,
    /// Southeast
    SouthEast,
    /// South
    South,
    /// Southwest
    SouthWest,
    /// West
    West,
    /// Northwest
    NorthWest,
}

impl Direction {
    /// Turns on this LED
    pub fn on(&self) {
        match *self {
            Direction::North => LEDS[0].on(),
            Direction::NorthEast => LEDS[1].on(),
            Direction::East => LEDS[2].on(),
            Direction::SouthEast => LEDS[3].on(),
            Direction::South => LEDS[4].on(),
            Direction::SouthWest => LEDS[5].on(),
            Direction::West => LEDS[6].on(),
            Direction::NorthWest => LEDS[7].on(),
        }
    }
}

/// Turns off all the LEDs
pub fn all_off() {
    for led in LEDS.iter() {
        led.off();
    }
}
