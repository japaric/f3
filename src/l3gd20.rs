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

use I16x3;
use peripheral;

// Registers
const CTRL_REG1: u8 = 0x20;
const OUT_X_L: u8 = 0x28;

unsafe fn read(register: u8, bytes: &mut [u8]) {
    const MS: u8 = 1 << 6;
    const READ: u8 = 1 << 7;

    let gpioe = peripheral::gpioe();
    let spi1 = peripheral::spi1_mut();

    // CS: low
    gpioe.bsrr.write(|w| w.br3(true));

    while !spi1.sr.read().txe() {}
    spi1.dr.write_u8(READ | MS | register);

    while !spi1.sr.read().rxne() {}
    spi1.dr.read_u8();

    for byte in bytes {
        while !spi1.sr.read().txe() {}
        spi1.dr.write_u8(0x0);

        while !spi1.sr.read().rxne() {}
        *byte = spi1.dr.read_u8();
    }

    // CS: high
    gpioe.bsrr.write(|w| w.bs3(true));
}

unsafe fn write(register: u8, value: u8) {
    const WRITE: u8 = 0 << 7;

    let gpioe = peripheral::gpioe();
    let spi1 = peripheral::spi1_mut();

    // CS: low
    gpioe.bsrr.write(|w| w.br3(true));

    while !spi1.sr.read().txe() {}
    spi1.dr.write_u8(WRITE | register);

    while !spi1.sr.read().txe() {}
    spi1.dr.write_u8(value);

    for _ in 0..2 {
        while !spi1.sr.read().rxne() {}
        spi1.dr.read_u8();
    }

    // CS: high
    gpioe.bsrr.write(|w| w.bs3(true));
}

/// Reads the angular rate
pub fn angular_rate() -> I16x3 {
    let mut bytes = [0; 6];

    unsafe {
        read(OUT_X_L, &mut bytes);
    }

    let out_x_l = u16::from(bytes[0]);
    let out_x_h = u16::from(bytes[1]);
    let out_y_l = u16::from(bytes[2]);
    let out_y_h = u16::from(bytes[3]);
    let out_z_l = u16::from(bytes[4]);
    let out_z_h = u16::from(bytes[5]);

    I16x3 {
        x: ((out_x_h << 8) + out_x_l) as i16,
        y: ((out_y_h << 8) + out_y_l) as i16,
        z: ((out_z_h << 8) + out_z_l) as i16,
    }
}

/// Initializes the SPI peripheral that's connected to the L3GD20
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let gpioa = peripheral::gpioa_mut();
    let gpioe = peripheral::gpioe_mut();
    let rcc = peripheral::rcc_mut();
    let spi1 = peripheral::spi1_mut();

    // RCC: enable GPIOA, GPIOE and SPI1
    rcc.ahbenr.modify(|_, w| w.iopaen(true).iopeen(true));
    rcc.apb2enr.modify(|_, w| w.spi1en(true));

    // GPIOA: configure PA5, PA6 and PA7 for SPI use
    // AFRL5 = 5 (SPI1_SCK)
    // AFRL6 = 5 (SPI1_MISO)
    // AFRL7 = 5 (SPI1_MOSI)
    // MODER* = 0b10 (Alternate function)
    gpioa.afrl.modify(|_, w| w.afrl5(5).afrl6(5).afrl7(5));
    gpioa.moder.modify(|_, w| w.moder5(0b10).moder6(0b10).moder7(0b10));

    // GPIOE: configure PE3 as output and drive it low to enable SPI mode
    gpioe.moder.modify(|_, w| w.moder3(0b01));
    gpioe.bsrr.write(|w| w.bs3(true));

    // SPI1: configuration
    // BIDIMODE + RXONLY: full-duplex (2 unidirectional lines)
    // SSM: enable software slave management
    // SSI: set NSS high
    // LSBFIRST: send the MSB first
    // SPE: enable the peripheral
    // BR: f_PCLK / 8 = 8 MHz / 8 = 1 MHz
    // MSTR: Master configuration
    // CPOL: SCK is 0 when idle (!)
    // CPHA: capture data on the first clock transition
    spi1.cr1.write(|w| {
        w.bidimode(false)
            .rxonly(false)
            .ssm(true)
            .ssi(true)
            .lsbfirst(false)
            .br(0b010)
            .mstr(true)
            .cpol(false)
            .cpha(false)
    });

    // FRXTH: RXNE threshold is 8-bit
    // DS: 8-bit data
    // SSOE: disable output on the NSS pin
    spi1.cr2.write(|w| w.frxth(true).ds(0b0111).frxth(true).ssoe(false));

    spi1.cr1.modify(|_, w| w.spe(true));

    // L3GD20: configure the gyroscope to operate at 380 Hz with a cut-off
    // frequency of 20 Hz
    write(CTRL_REG1, 0b1000_1111);
}
