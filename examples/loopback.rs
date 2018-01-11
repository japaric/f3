// NOTE you MUST short the TX and RX pins to successfully run this program

#![no_std]

extern crate cortex_m;
extern crate f3;
#[macro_use(block)]
extern crate nb;

use f3::hal::prelude::*;
use f3::hal::serial::Serial;
use f3::hal::stm32f30x;

fn main() {
    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constraint();
    let mut rcc = p.RCC.constraint();

    // TRY the other clock configuration
    let clocks = rcc.CFGR.freeze(&mut flash.ACR);
    // let clocks = rcc.CFGR.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.ACR);

    let mut gpioa = p.GPIOA.split(&mut rcc.AHB);
    let mut gpiob = p.GPIOB.split(&mut rcc.AHB);

    // TRY any of these TX pins
    let tx = gpioa.PA9.as_af7(&mut gpioa.MODER, &mut gpioa.AFRH);
    // let tx = gpiob.PB6.as_af7(&mut gpiob.MODER, &mut gpiob.AFRL);

    // TRY any of these RX pins
    // let rx = gpioa.PA10.as_af7(&mut gpioa.MODER, &mut gpioa.AFRH);
    let rx = gpiob.PB7.as_af7(&mut gpiob.MODER, &mut gpiob.AFRL);

    // TRY using a different USART
    let serial = Serial::usart1(p.USART1, 9_600.bps(), (tx, rx), &mut rcc.APB2, clocks);
    let (mut tx, mut rx) = serial.split();

    cortex_m::asm::bkpt();

    block!(tx.write(b'X')).unwrap();
    assert_eq!(block!(rx.read()).unwrap(), b'X');

    cortex_m::asm::bkpt();
}
