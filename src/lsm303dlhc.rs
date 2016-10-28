//! LSM303DLHC - Accelerometer + Magnetometer
//!
//! This sensor is connected to the I2C bus via these pins:
//!
//! - Clock (`SCL`) pin - `PB6`
//! - Data (`SDA`) pin - `PB7`
//!
//! # Accelerometer
//!
//! - Sample rate: `400 Hz`
//! - Input range: `[-8g, +8g]`
//! - Gain: `8 mg / LSB`
//! - Output range: 16 bits (`i16`)
//!
//! # Magnetometer
//!
//! - Sample rate: `15 Hz`
//! - Input range: `[-1.3G, +1.3G]`
//! - Gain X, Y: `1100 LSB / Gauss`
//! - Gain Z: `980 LSB / Gauss`
//! - Output range: 12 bits (`[-2048, +2047]`)

use futuro::prelude::Future;

// Slave addresses
const ACCELEROMETER: u8 = 0b001_1001;
const MAGNETOMETER: u8 = 0b001_1110;

// Register addresses
const CTRL_REG1_A: u8 = 0x20;
const CTRL_REG4_A: u8 = 0x23;
const MR_REG_M: u8 = 0x02;
const OUT_X_H_M: u8 = 0x3;
const OUT_X_L_A: u8 = 0x28;
const MULTI_READ: u8 = 1 << 7;

use i2c::{Address, Free, I2c};

/// Acceleration reading
pub struct Acceleration {
    buffer: [u8; 6],
}

impl Acceleration {
    /// Computes the X component
    pub fn x(&self) -> i16 {
        let out_x_l_a = u16::from(self.buffer[0]);
        let out_x_h_a = u16::from(self.buffer[1]);

        ((out_x_h_a << 8) + out_x_l_a) as i16
    }

    /// Computes the Y component
    pub fn y(&self) -> i16 {
        let out_y_l_a = u16::from(self.buffer[2]);
        let out_y_h_a = u16::from(self.buffer[3]);

        ((out_y_h_a << 8) + out_y_l_a) as i16
    }

    /// Computes the Z component
    pub fn z(&self) -> i16 {
        let out_z_l_a = u16::from(self.buffer[4]);
        let out_z_h_a = u16::from(self.buffer[5]);

        ((out_z_h_a << 8) + out_z_l_a) as i16
    }
}

/// Magnetic field reading
pub struct MagneticField {
    buffer: [u8; 6],
}

impl MagneticField {
    /// Computes the X component
    pub fn x(&self) -> i16 {
        let out_x_h_m = u16::from(self.buffer[0]);
        let out_x_l_m = u16::from(self.buffer[1]);

        ((out_x_h_m << 8) + out_x_l_m) as i16
    }

    /// Computes the Y component
    pub fn y(&self) -> i16 {
        let out_y_h_m = u16::from(self.buffer[4]);
        let out_y_l_m = u16::from(self.buffer[5]);

        ((out_y_h_m << 8) + out_y_l_m) as i16
    }

    /// Computes the Z component
    pub fn z(&self) -> i16 {
        let out_z_h_m = u16::from(self.buffer[2]);
        let out_z_l_m = u16::from(self.buffer[3]);

        ((out_z_h_m << 8) + out_z_l_m) as i16
    }
}

/// LSM303DLHC
pub struct Lsm303dlhc {
    i2c: I2c<Free>,
}

impl Lsm303dlhc {
    /// Connects to and initializes the LSM303DLHC
    pub fn new(i2c: I2c<Free>) -> Lsm303dlhc {
        // configure the accelerometer to operate at 400 Hz
        // configure the accelerometer to operate in the [-8g, +8g] range
        // configure the magnetometer to operate in continuous mode
        let (i2c, mut register) =
            i2c.write_all(Address::u7(ACCELEROMETER),
                           [CTRL_REG1_A, 0b0111_0111])
                .wait()
                .write(None, CTRL_REG4_A)
                .wait()
                .read(None)
                .wait();

        register &= !(0b11 << 4);
        register |= 0b10 << 4;

        let i2c = i2c.write_all(None, [CTRL_REG4_A, register])
            .wait()
            .stop()
            .wait()
            .write_all(Address::u7(MAGNETOMETER), [MR_REG_M, 0b00])
            .wait()
            .stop()
            .wait();

        Lsm303dlhc { i2c: i2c }
    }

    /// Reads the acceleration
    pub fn acceleration(self) -> impl Future<Item = (Self, Acceleration)> {
        self.i2c
            .write(Address::u7(ACCELEROMETER), MULTI_READ | OUT_X_L_A)
            .and_then(|i2c| i2c.read_exact(None, [0; 6]))
            .and_then(|(i2c, buffer)| {
                i2c.stop().map(move |i2c| {

                    (Lsm303dlhc { i2c: i2c }, Acceleration { buffer: buffer })
                })
            })
    }

    /// Reads the magnetic field
    pub fn magnetic_field(self) -> impl Future<Item = (Self, MagneticField)> {
        self.i2c
            .write(Address::u7(MAGNETOMETER), OUT_X_H_M)
            .and_then(|i2c| i2c.read_exact(None, [0; 6]))
            .and_then(|(i2c, buffer)| {
                i2c.stop().map(move |i2c| {

                    (Lsm303dlhc { i2c: i2c }, MagneticField { buffer: buffer })
                })
            })
    }
}
