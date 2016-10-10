//! L3GD20 - SPI
//!
//! - Master Input (`MISO`) pin - `PA6`
//! - Master Output (`MOSI`) pin - `PA7`
//! - Serial Clock (`SCK`) pin - `PA5`
//! - `CS` pin - `PE3`

use peripheral;

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
}
