#![no_std]

extern crate cortex_m;
extern crate f3;
#[macro_use(block)]
extern crate nb;

use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::hal::timer::Timer;
use f3::led::Led;

fn main() {
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constraint();
    let mut flash = dp.FLASH.constraint();

    // TRY the other clock configuration
    // let clocks = rcc.CFGR.freeze(&mut flash.ACR);
    let clocks = rcc.CFGR.sysclk(16.mhz()).freeze(&mut flash.ACR);

    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);

    // TRY the other timers
    let mut timer = Timer::tim4(dp.TIM4, 1.hz(), clocks, &mut rcc.APB1);
    let mut led: Led = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();

    timer.resume();
    loop {
        block!(timer.wait()).unwrap();
        led.on();
        block!(timer.wait()).unwrap();
        led.off();
    }
}
