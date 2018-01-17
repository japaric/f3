//! Cooperative multitasking
//!
//! This programs runs the `roulette` and `serial-echo` examples as concurrent tasks using generators.
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![feature(generator_trait)]
//! #![feature(generators)]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! extern crate f3;
//! #[macro_use(await)]
//! extern crate nb;
//! 
//! use core::ops::Generator;
//! 
//! use f3::hal::prelude::*;
//! use f3::hal::serial::Serial;
//! use f3::hal::stm32f30x;
//! use f3::hal::timer::Timer;
//! use f3::led::Leds;
//! 
//! fn main() {
//!     let p = stm32f30x::Peripherals::take().unwrap();
//! 
//!     let mut flash = p.FLASH.constrain();
//!     let mut rcc = p.RCC.constrain();
//!     let gpioe = p.GPIOE.split(&mut rcc.ahb);
//!     let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
//! 
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//! 
//!     let tx = gpioa.pa9.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
//!     let rx = gpioa.pa10.into_af7(&mut gpioa.moder, &mut gpioa.afrh);
//! 
//!     let serial = Serial::usart1(p.USART1, (tx, rx), 115_200.bps(), clocks, &mut rcc.apb2);
//!     let (mut tx, mut rx) = serial.split();
//! 
//!     let mut leds: Leds = Leds::new(gpioe);
//!     // This timer, unlike `delay`, does *periodic* count downs at specified frequency (8 Hz)
//!     let mut timer = Timer::tim6(p.TIM6, 8.hz(), clocks, &mut rcc.apb1);
//! 
//!     // Tasks
//!     let mut roulette = || {
//!         let n = leds.len();
//!         loop {
//!             for curr in 0..n {
//!                 let next = (curr + 1) % n;
//!                 leds[curr].off();
//!                 leds[next].on();
//! 
//!                 // NOTE error type is `!`
//!                 await!(timer.wait()).ok();
//!             }
//!         }
//!     };
//! 
//!     let mut echo = || {
//!         loop {
//!             let byte = await!(rx.read()).unwrap();
//!             // NOTE error type is `!`
//!             await!(tx.write(byte)).ok();
//!         }
//!     };
//! 
//!     // Run tasks cooperatively
//!     // When a task can't do any more progress it suspends it execution and the next task is resumed
//!     loop {
//!         roulette.resume();
//!         echo.resume();
//!     }
//! }
//! ```
// Auto-generated. Do not modify.
