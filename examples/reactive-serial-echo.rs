//! Serial interface echo server (reactive version)
//!
//! This is a reactive version of the `serial-echo` example. Here the processor sleeps most of the
//! time and only wakes up to sent back received bytes.
//!
//! This example uses the [Real Time For the Masses framework](https://docs.rs/cortex-m-rtfm/~0.3)
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate f3;

use f3::hal::prelude::*;
use f3::hal::serial::{Event, Rx, Serial, Tx};
use f3::hal::stm32f30x::{self, USART1};
use rtfm::{app, Threshold};

app! {
    device: stm32f30x,

    resources: {
        static TX: Tx<USART1>;
        static RX: Rx<USART1>;
    },

    tasks: {
        USART1_EXTI25: {
            path: echo,
            resources: [TX, RX],
        }
    },
}

fn init(p: init::Peripherals) -> init::LateResources {
    let mut flash = p.device.FLASH.constrain();
    let mut rcc = p.device.RCC.constrain();
    let mut gpioa = p.device.GPIOA.split(&mut rcc.ahb);

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
    let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);

    let mut serial = Serial::usart1(
        p.device.USART1,
        (tx, rx),
        115_200.bps(),
        clocks,
        &mut rcc.apb2,
    );
    serial.listen(Event::Rxne);
    let (tx, rx) = serial.split();

    init::LateResources { TX: tx, RX: rx }
}

fn idle() -> ! {
    // sleep
    loop {
        rtfm::wfi();
    }
}

fn echo(_: &mut Threshold, mut r: USART1_EXTI25::Resources) {
    let byte = r.RX.read().unwrap();
    r.TX.write(byte).unwrap();
}
