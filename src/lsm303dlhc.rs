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

use ref_slice;

use I16x3;
use peripheral;

// Slave addresses
const ACCELEROMETER: u8 = 0b001_1001;
const MAGNETOMETER: u8 = 0b001_1110;

// Register addresses
const CTRL_REG1_A: u8 = 0x20;
const CTRL_REG4_A: u8 = 0x23;
const MR_REG_M: u8 = 0x02;
const OUT_X_H_M: u8 = 0x3;
const OUT_X_L_A: u8 = 0x28;

unsafe fn read(slave: u8, register: u8, bytes: &mut [u8]) {
    let i2c1 = peripheral::i2c1_mut();
    i2c1.cr2.write(|w| {
        w.sadd1(slave).rd_wrn(false).nbytes(1).start(true).autoend(false)
    });

    while !i2c1.isr.read().txis() {}
    i2c1.txdr.write(|w| w.txdata(register));

    while !i2c1.isr.read().tc() {}

    i2c1.cr2.modify(|_, w| {
        w.nbytes(bytes.len() as u8).rd_wrn(true).start(true).autoend(true)
    });

    for byte in bytes {
        while !i2c1.isr.read().rxne() {}

        *byte = i2c1.rxdr.read().rxdata();
    }
}

unsafe fn write(slave: u8, register: u8, value: u8) {
    let i2c1 = peripheral::i2c1_mut();
    i2c1.cr2.write(|w| {
        w.sadd1(slave).rd_wrn(false).nbytes(2).start(true).autoend(true)
    });

    while !i2c1.isr.read().txis() {}
    i2c1.txdr.write(|w| w.txdata(register));

    while !i2c1.isr.read().txis() {}
    i2c1.txdr.write(|w| w.txdata(value));
}

/// Reads the acceleration
pub fn acceleration() -> I16x3 {
    const MULTI_READ: u8 = 1 << 7;

    let mut bytes = [0; 6];
    unsafe {
        read(ACCELEROMETER, MULTI_READ | OUT_X_L_A, &mut bytes);
    }

    let out_x_l_a = u16::from(bytes[0]);
    let out_x_h_a = u16::from(bytes[1]);
    let out_y_l_a = u16::from(bytes[2]);
    let out_y_h_a = u16::from(bytes[3]);
    let out_z_l_a = u16::from(bytes[4]);
    let out_z_h_a = u16::from(bytes[5]);

    I16x3 {
        x: ((out_x_h_a << 8) + out_x_l_a) as i16,
        y: ((out_y_h_a << 8) + out_y_l_a) as i16,
        z: ((out_z_h_a << 8) + out_z_l_a) as i16,
    }
}

/// Reads the magnetic field
pub fn magnetic_field() -> I16x3 {
    let mut bytes = [0; 6];

    unsafe {
        read(MAGNETOMETER, OUT_X_H_M, &mut bytes);
    }

    let out_x_h_m = u16::from(bytes[0]);
    let out_x_l_m = u16::from(bytes[1]);
    let out_z_h_m = u16::from(bytes[2]);
    let out_z_l_m = u16::from(bytes[3]);
    let out_y_h_m = u16::from(bytes[4]);
    let out_y_l_m = u16::from(bytes[5]);

    I16x3 {
        x: ((out_x_h_m << 8) + out_x_l_m) as i16,
        y: ((out_y_h_m << 8) + out_y_l_m) as i16,
        z: ((out_z_h_m << 8) + out_z_l_m) as i16,
    }
}

/// Initializes the LSM303DLHC
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let gpiob = peripheral::gpiob_mut();
    let i2c1 = peripheral::i2c1_mut();
    let rcc = peripheral::rcc_mut();

    // RCC: enable the I2C1 peripheral
    rcc.ahbenr.modify(|_, w| w.iopben(true));
    rcc.apb1enr.modify(|_, w| w.i2c1en(true));

    // GPIOB: configure PB6 and PB7 for I2C use
    // AFRL6 = 4 (I2C1_SCL)
    // AFRL7 = 4 (I2C1_SDA)
    // MODER* = 0b10 (Alternate function)
    gpiob.afrl.modify(|_, w| w.afrl6(4).afrl7(4));
    gpiob.moder.modify(|_, w| w.moder6(0b10).moder7(0b10));

    // Configure for "fast mode" (400 KHz)
    // PRESC:  t_I2CCLK = (0 + 1) / 8 MHz = 125 ns
    // SCLL:   t_SCLL   = (9 + 1) * t_I2CCLK = 1.25 us
    // SCLH:   t_SCLH   = (3 + 1) * t_I2CCLK = 0.5 us
    //
    // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK = 0.5 us
    // t_SCL = t_SYNC1 + t_SYNC2 t_SCLL + t_SCLH ~= 2.5 us
    i2c1.timingr.write(|w| w.presc(0).scll(9).sclh(3).sdadel(1).scldel(3));
    i2c1.cr2.write(|w| w.add10(false));

    // Enable the peripheral
    i2c1.cr1.write(|w| w.pe(true));

    // LSM303DLHC: configure the accelerometer to operate at 400 Hz
    write(ACCELEROMETER, CTRL_REG1_A, 0b0111_0111);

    // LSM303DLHC: configure the accelerometer to operate in the [-8g, +8g]
    // range
    // TODO use the `ref_slice` crate
    let mut register = 0u8;
    read(ACCELEROMETER,
         CTRL_REG4_A,
         ref_slice::ref_slice_mut(&mut register));
    register &= !(0b11 << 4);
    register |= 0b10 << 4;
    write(ACCELEROMETER, CTRL_REG4_A, register);

    // LSM303DLHC: configure the magnetometer to operate in continuous mode
    write(MAGNETOMETER, MR_REG_M, 0b00);
}
