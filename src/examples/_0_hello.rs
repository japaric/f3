//! Prints "Hello" and then "World" on the OpenOCD console
//!
//! ```
//! 
//! #![feature(used)]
//! #![no_std]
//! 
//! // version = "0.2.6"
//! #[macro_use]
//! extern crate cortex_m;
//! 
//! // version = "0.2.0"
//! extern crate cortex_m_rt;
//! 
//! // version = "0.1.0"
//! #[macro_use]
//! extern crate cortex_m_rtfm as rtfm;
//! 
//! extern crate f3;
//! 
//! use f3::stm32f30x;
//! use rtfm::{P0, T0, TMax};
//! 
//! // TASKS
//! tasks!(stm32f30x, {});
//! 
//! // INITIALIZATION PHASE
//! fn init(_priority: P0, _threshold: &TMax) {
//!     hprintln!("Hello");
//! }
//! 
//! // IDLE LOOP
//! fn idle(_priority: P0, _threshold: T0) -> ! {
//!     hprintln!("World");
//! 
//!     // Sleep
//!     loop {
//!         rtfm::wfi();
//!     }
//! }
//! ```
// Auto-generated. Do not modify.
