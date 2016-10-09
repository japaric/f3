//! I2C
//!
//! SCL pin - `PB6`
//! SDA pin - `PB7`

use peripheral;

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
}
