#![no_std]

extern crate cortex_m;
extern crate f3;

use f3::prelude::*;
use f3::stm32f30x;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = p.RCC.split();
    let mut gpioe = p.GPIOE.split(&mut rcc.AHB);
    let mut ld3 = gpioe
        .PE9
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let mut ld4 = gpioe
        .PE8
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let mut ld5 = gpioe
        .PE10
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let mut ld6 = gpioe
        .PE15
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let mut ld7 = gpioe
        .PE11
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let mut ld8 = gpioe
        .PE14
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let mut ld9 = gpioe
        .PE12
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);
    let mut ld10 = gpioe
        .PE13
        .as_push_pull_output(&mut gpioe.MODER, &mut gpioe.OTYPER);

    ld3.set_high();
    cortex_m::asm::bkpt();
    ld4.set_high();
    cortex_m::asm::bkpt();
    ld5.set_high();
    cortex_m::asm::bkpt();
    ld6.set_high();
    cortex_m::asm::bkpt();
    ld7.set_high();
    cortex_m::asm::bkpt();
    ld8.set_high();
    cortex_m::asm::bkpt();
    ld9.set_high();
    cortex_m::asm::bkpt();
    ld10.set_high();
    cortex_m::asm::bkpt();
}
