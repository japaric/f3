#![no_std]

extern crate cortex_m;
extern crate f3;

use f3::hal::delay::Delay;
use f3::led::Led;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;

fn main() {
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constraint();
    let mut flash = dp.FLASH.constraint();

    // Try the other clock configuration
    // let clocks = rcc.CFGR.freeze(&mut flash.ACR);
    let clocks = rcc.CFGR.sysclk(16.mhz()).freeze(&mut flash.ACR);

    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);

    let mut delay = Delay::new(cp.SYST, clocks);
    let mut led: Led = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();

    let half_period = 1_000_u32;
    loop {
        led.on();
        delay.delay_ms(half_period);
        led.off();
        delay.delay_ms(half_period);
    }
}
