//! A LED roulette!
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m_rt::entry;
use f3::{
    hal::{delay::Delay, prelude::*, stm32f30x},
    led::Leds,
};

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (all clocks run at 16 MHz)
    // let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(cp.SYST, clocks);

    let n = leds.len();
    loop {
        for curr in 0..n {
            let next = (curr + 1) % n;
            let mut result = leds[curr].off();
            assert_eq!(result.is_err(),false);
            result = leds[next].on();
            assert_eq!(result.is_err(),false);

            delay.delay_ms(100_u8);
        }
    }
}
