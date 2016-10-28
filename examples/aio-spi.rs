#![no_main]
#![no_std]

#[macro_use]
extern crate f3;
extern crate futuro;

use f3::peripheral;
use f3::spi::Spi;
use futuro::prelude::*;

#[no_mangle]
pub fn main() -> ! {
    // WHO_AM_I
    const REGISTER_ADDRESS: u8 = 0x0f;
    // Read the register
    const READ: u8 = 1 << 7;

    let spi = Spi::new().unwrap();
    let rcc = unsafe { peripheral::rcc_mut() };
    let gpioe = unsafe { peripheral::gpioe_mut() };

    rcc.ahbenr.modify(|_, w| w.iopeen(true));
    gpioe.moder.modify(|_, w| w.moder3(0b01));
    gpioe.bsrr.write(|w| w.br3(true));

    let buffer = spi.transfer_all([READ | REGISTER_ADDRESS, 0b00]).wait().1;

    gpioe.bsrr.write(|w| w.bs3(true));

    // Expected output: 0x0f - 0xd4
    iprintln!("0x{:02x} - 0x{:02x}", REGISTER_ADDRESS, buffer[1]);

    loop {
    }
}
