#![no_std]

extern crate cortex_m;
extern crate f3;

use f3::delay::Delay;
use f3::led::Led;
use f3::prelude::*;
use f3::stm32f30x;

fn main() {
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.split();
    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);

    let mut delay = Delay::new(cp.SYST);
    let mut led: Led = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();

    let half_period = 1_000;
    loop {
        led.on();
        delay.ms(half_period);
        led.off();
        delay.ms(half_period);
    }
}
