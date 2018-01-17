//! Preemptive multitasking
//!
//! This programs runs the `roulette` and `serial-echo` examples as concurrent tasks using
//! preemption. The `serial-echo` task is given higher priority and can preempt the other task.
//!
//! This example uses the [Real Time For the Masses framework](https://docs.rs/cortex-m-rtfm/~0.3)
#![deny(unsafe_code)]
#![deny(warnings)]
#![feature(proc_macro)]
#![no_std]

extern crate cortex_m_rtfm as rtfm;
extern crate f3;

use f3::hal::delay::Delay;
use f3::hal::prelude::*;
use f3::hal::serial::{Event, Rx, Serial, Tx};
use f3::hal::stm32f30x::{self, USART1};
use f3::led::Leds;
use rtfm::{app, Threshold};

app! {
    device: stm32f30x,

    resources: {
        static DELAY: Delay;
        static LEDS: Leds;
        static RX: Rx<USART1>;
        static TX: Tx<USART1>;
    },

    idle: {
        resources: [DELAY, LEDS],
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

    let delay = Delay::new(p.core.SYST, clocks);
    let leds = Leds::new(p.device.GPIOE.split(&mut rcc.ahb));

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

    init::LateResources {
        DELAY: delay,
        LEDS: leds,
        RX: rx,
        TX: tx,
    }
}

fn idle(_t: &mut Threshold, r: idle::Resources) -> ! {
    loop {
        let n = r.LEDS.len();
        for curr in 0..n {
            let next = (curr + 1) % n;

            r.LEDS[curr].off();
            r.LEDS[next].on();

            r.DELAY.delay_ms(125_u8);
        }
    }
}

fn echo(_: &mut Threshold, mut r: USART1_EXTI25::Resources) {
    let byte = r.RX.read().unwrap();
    r.TX.write(byte).unwrap();
}
