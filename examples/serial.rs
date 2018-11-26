//! Test the serial interface
//!
//! This example requires you to short (connect) the TX and RX pins.
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate panic_semihosting;

use cortex_m::asm;
use cortex_m_rt::entry;
use f3::hal::{prelude::*, serial::Serial, stm32f30x};
use nb::block;

#[entry]
fn main() -> ! {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut gpioc = p.GPIOC.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    let tx = gpioc.pc4.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    let rx = gpioc.pc5.into_af7(&mut gpioc.moder, &mut gpioc.afrl);

    // TRY using a different USART peripheral here
    let serial = Serial::usart1(p.USART1, (tx, rx), 9_600.bps(), clocks, &mut rcc.apb2);
    let (mut tx, mut _rx) = serial.split();

    let sent = b'X';

    asm::bkpt();

    // The `block!` macro makes an operation block until it finishes
    // NOTE the error type is `!`
    block!(tx.write(sent)).ok();

    // let received = block!(_rx.read()).unwrap();

    // assert_eq!(received, sent);

    // if all goes well you should reach this breakpoint
    asm::bkpt();

    loop {}
}
