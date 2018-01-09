#![no_std]

extern crate cortex_m;
extern crate f3;

use f3::prelude::*;
use f3::stm32f30x;
use f3::delay::Delay;

fn main() {
    let dp = stm32f30x::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.split();
    let mut gpioe = dp.GPIOE.split(&mut rcc.AHB);

    let mut delay = Delay::new(cp.SYST);
    let mut ld3 = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);

    loop {
        ld3.set_high();
        delay.ms(1_000);
        ld3.set_low();
        delay.ms(1_000);
    }
}
