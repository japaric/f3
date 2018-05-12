//! Interfacing the on-board L3GD20 (gyroscope)
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate f3;
extern crate panic_semihosting;

use cortex_m::asm;
use f3::hal::prelude::*;
use f3::hal::spi::Spi;
use f3::hal::stm32f30x;
use f3::{l3gd20, L3gd20};
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();

    // TRY the other clock configuration
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
    let mut gpioe = p.GPIOE.split(&mut rcc.ahb);

    let mut nss = gpioe
        .pe3
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
    nss.set_high();

    // The `L3gd20` abstraction exposed by the `f3` crate requires a specific pin configuration to
    // be used and won't accept any configuration other than the one used here. Trying to use a
    // different pin configuration will result in a compiler error.
    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        l3gd20::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    let mut l3gd20 = L3gd20::new(spi, nss).unwrap();

    // sanity check: the WHO_AM_I register always contains this value
    assert_eq!(l3gd20.who_am_i().unwrap(), 0xD4);

    let _m = l3gd20.all().unwrap();

    // when you reach this breakpoint you'll be able to inspect the variable `_m` which contains the
    // gyroscope and the temperature sensor readings
    asm::bkpt();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
