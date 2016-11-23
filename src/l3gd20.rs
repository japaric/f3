//! L3GD20 - Gyroscope
//!
//! This sensor is connected to the SPI bus via these pins:
//!
//! - Master Input (`MISO`) pin - `PA6`
//! - Master Output (`MOSI`) pin - `PA7`
//! - Serial Clock (`SCK`) pin - `PA5`
//! - `CS` pin - `PE3`
//!
//! # Gyroscope
//!
//! - Sample rate: `380 Hz`
//! - Input range: `[-250 dps, +250 dps]`
//! - Gain: `8.75e-3 dps / LSB`
//! - Output range: 16 bits (`i16`)

use futuro::prelude::Future;

use peripheral;
use spi::Spi;

// Registers
const CTRL_REG1: u8 = 0x20;
const OUT_X_L: u8 = 0x28;

const MS: u8 = 1 << 6;
const READ: u8 = 1 << 7;
const WRITE: u8 = 0 << 7;

/// L3GD20
pub struct L3gd20 {
    spi: Spi,
}

/// Angular rate reading
pub struct AngularRate {
    buffer: [u8; 6],
}

impl AngularRate {
    /// Computes the X component
    pub fn x(&self) -> i16 {
        let out_x_l = u16::from(self.buffer[0]);
        let out_x_h = u16::from(self.buffer[1]);

        ((out_x_h << 8) + out_x_l) as i16
    }

    /// Computes the Y component
    pub fn y(&self) -> i16 {
        let out_y_l = u16::from(self.buffer[2]);
        let out_y_h = u16::from(self.buffer[3]);

        ((out_y_h << 8) + out_y_l) as i16
    }

    /// Computes the Z component
    pub fn z(&self) -> i16 {
        let out_z_l = u16::from(self.buffer[4]);
        let out_z_h = u16::from(self.buffer[5]);

        ((out_z_h << 8) + out_z_l) as i16
    }
}

impl L3gd20 {
    /// Connects to and initializes the L3GD20
    pub fn new(spi: Spi) -> L3gd20 {
        let gpioe = unsafe { peripheral::gpioe_mut() };
        let rcc = unsafe { peripheral::rcc_mut() };

        // enable GPIOE
        rcc.ahbenr.modify(|_, w| w.iopeen(true));

        // configure PE3 as output and set it high (SPI disabled)
        gpioe.moder.modify(|_, w| w.moder3(0b01));

        // enable SPI
        gpioe.bsrr.write(|w| w.br3(true));

        // configure the gyroscope to operate at 380 Hz with a cut-off
        // frequency of 20 Hz
        spi.transfer_all([WRITE | CTRL_REG1, 0b1000_1111])
            .map(|(spi, _)| {
                // disable SPI
                gpioe.bsrr.write(|w| w.bs3(true));
                L3gd20 { spi: spi }
            })
            .wait()
    }

    /// Reads the angular rate
    pub fn angular_rate(self) -> impl Future<Item = (Self, AngularRate)> {
        let gpioe = unsafe { peripheral::gpioe_mut() };

        // enable SPI
        gpioe.bsrr.write(|w| w.br3(true));
        self.spi
            .transfer_all([READ | MS | OUT_X_L, 0, 0, 0, 0, 0, 0])
            .map(|(spi, buffer)| {
                let gpioe = unsafe { peripheral::gpioe_mut() };

                // disable SPI
                gpioe.bsrr.write(|w| w.bs3(true));


                (L3gd20 { spi: spi },
                 AngularRate {
                     buffer: [buffer[1], buffer[2], buffer[3], buffer[4],
                              buffer[5], buffer[6]],
                 })
            })
    }
}
