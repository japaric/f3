//! Turns all the user LEDs on
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_std]
//! #![no_main]
//! 
//! extern crate f3;
//! #[macro_use(entry, exception)]
//! extern crate cortex_m_rt as rt;
//! extern crate panic_semihosting;
//! 
//! use f3::hal::prelude::*;
//! use f3::hal::stm32f30x;
//! use f3::led::Leds;
//! use rt::ExceptionFrame;
//! 
//! entry!(main);
//! 
//! fn main() -> ! {
//!     let p = stm32f30x::Peripherals::take().unwrap();
//! 
//!     let mut rcc = p.RCC.constrain();
//!     let gpioe = p.GPIOE.split(&mut rcc.ahb);
//! 
//!     let mut leds = Leds::new(gpioe);
//! 
//!     for led in leds.iter_mut() {
//!         led.on();
//!     }
//! 
//!     loop {}
//! }
//! 
//! exception!(HardFault, hard_fault);
//! 
//! fn hard_fault(ef: &ExceptionFrame) -> ! {
//!     panic!("{:#?}", ef);
//! }
//! 
//! exception!(*, default_handler);
//! 
//! fn default_handler(irqn: i16) {
//!     panic!("Unhandled exception (IRQn = {})", irqn);
//! }
//! ```
// Auto-generated. Do not modify.
