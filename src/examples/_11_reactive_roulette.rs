//! A LED roulette! (reactive version)
//!
//! This is a reactive version of the `roulette` example. Here the processor sleeps most of the time
//! and only wakes up to advance the roulette.
//!
//! This example uses the [Real Time For the Masses framework](https://docs.rs/cortex-m-rtfm/~0.3)
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![feature(proc_macro)]
//! #![no_std]
//! 
//! extern crate cortex_m;
//! extern crate cortex_m_rtfm as rtfm;
//! extern crate f3;
//! 
//! use f3::hal::prelude::*;
//! use f3::hal::stm32f30x::{self, TIM6};
//! use f3::hal::timer::{Event, Timer};
//! use f3::led::Leds;
//! use rtfm::{app, Threshold};
//! 
//! app! {
//!     device: stm32f30x,
//! 
//!     resources: {
//!         static LEDS: Leds;
//!         static TIMER: Timer<TIM6>;
//!         static CURR: usize = 0;
//!     },
//! 
//!     tasks: {
//!         TIM6_DACUNDER: {
//!             path: roulette,
//!             resources: [LEDS, TIMER, CURR],
//!         }
//!     },
//! }
//! 
//! fn init(p: init::Peripherals, _r: init::Resources) -> init::LateResources {
//!     let mut flash = p.device.FLASH.constrain();
//!     let mut rcc = p.device.RCC.constrain();
//!     let gpioe = p.device.GPIOE.split(&mut rcc.ahb);
//! 
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//! 
//!     let leds = Leds::new(gpioe);
//!     // timer that generates periodic (8 Hz) interrupts
//!     let mut timer = Timer::tim6(p.device.TIM6, 8.hz(), clocks, &mut rcc.apb1);
//!     timer.listen(Event::TimeOut);
//! 
//!     init::LateResources {
//!         LEDS: leds,
//!         TIMER: timer,
//!     }
//! }
//! 
//! fn idle() -> ! {
//!     // Sleep
//!     loop {
//!         rtfm::wfi();
//!     }
//! }
//! 
//! fn roulette(_: &mut Threshold, mut r: TIM6_DACUNDER::Resources) {
//!     // clear "expired" flag or we'll re-enter this interrupt
//!     r.TIMER.wait().unwrap();
//! 
//!     let curr = *r.CURR;
//!     let next = (curr + 1) % r.LEDS.len();
//!     r.LEDS[curr].off();
//!     r.LEDS[next].on();
//! 
//!     *r.CURR = next;
//! }
//! ```
// Auto-generated. Do not modify.
