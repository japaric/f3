#![no_std]

extern crate cortex_m;
extern crate f3;
#[macro_use(block)]
extern crate nb;

use f3::led::Led;
use f3::prelude::*;
use f3::stm32f30x;
use f3::timer::Timer;

fn main() {
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constraint();
    let mut flash = dp.FLASH.constraint();

    // Try the other clock configuration
    // let clocks = rcc.CFGR.freeze(&mut flash.ACR);
    let clocks = rcc.CFGR.sysclk(16.mhz()).freeze(&mut flash.ACR);

    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);

    let mut timer = Timer::new(dp.TIM6, 1.hz(), clocks, &mut rcc.APB1);
    let mut led: Led = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();

    timer.resume();
    loop {
        led.on();
        block!(timer.wait()).unwrap();
        led.off();
        block!(timer.wait()).unwrap();
    }
}
