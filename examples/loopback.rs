#![no_std]

extern crate cortex_m;
extern crate f3;
#[macro_use(block)]
extern crate nb;

use f3::prelude::*;
use f3::stm32f30x;
use f3::serial::Serial;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut rcc = p.RCC.split();
    let mut gpioa = p.GPIOA.split(&mut rcc.AHB);
    let tx = gpioa.PA9.as_af7(&mut gpioa.MODER, &mut gpioa.AFRH);
    let rx = gpioa.PA10.as_af7(&mut gpioa.MODER, &mut gpioa.AFRH);

    let serial = Serial::new(p.USART1, (tx, rx), &mut rcc.APB2, 9_600.bps());
    let (mut tx, mut rx) = serial.split();

    cortex_m::asm::bkpt();

    block!(tx.write(b'X')).unwrap();
    assert_eq!(block!(rx.read()).unwrap(), b'X');

    cortex_m::asm::bkpt();
}
