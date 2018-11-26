//! Blinks an LED
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

// Panic handler
extern crate panic_semihosting;

use cortex_m_rt::entry;
use f3::{
    hal::{delay::Delay, prelude::*, stm32f30x},
    led::Led,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (all clocks run at 16 MHz)
    // let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    let mut led: Led = gpioe
        .pe9
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper)
        .into();
    let mut delay = Delay::new(cp.SYST, clocks);

    loop {
        led.on();
        delay.delay_ms(1_000_u16);
        led.off();
        delay.delay_ms(1_000_u16);
    }
}
