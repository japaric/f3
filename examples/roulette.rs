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

    let mut flash = dp.FLASH.constraint();
    let mut rcc = dp.RCC.constraint();

    // Try the other clock configuration
    let clocks = rcc.CFGR.freeze(&mut flash.ACR);
    // let clocks = rcc.CFGR.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.ACR);

    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);
    let mut delay = Delay::new(cp.SYST, clocks);

    let n: Led = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let ne = gpioe
        .PE10
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let e = gpioe
        .PE11
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let se = gpioe
        .PE12
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let s = gpioe
        .PE13
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let sw = gpioe
        .PE14
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let w = gpioe
        .PE15
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();
    let nw = gpioe
        .PE8
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER)
        .into();

    let mut leds = [n, ne, e, se, s, sw, w, nw];

    let ms = 50_u8;
    loop {
        for curr in 0..8 {
            let next = (curr + 1) % 8;

            leds[next].on();
            delay.delay_ms(ms);
            leds[curr].off();
            delay.delay_ms(ms);
        }
    }
}
