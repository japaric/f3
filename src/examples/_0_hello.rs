//! Prints Hello and then World on the OpenOCD console
//!
//! ```
//! 
//! #![feature(used)]
//! #![no_std]
//! 
//! // version = "0.2.4"
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
//! use rtfm::{C0, C16, P0};
//! 
//! // INITIALIZATION PHASE
//! fn init(_prio: P0, _ceil: &C16) {
//!     hprintln!("Hello");
//! }
//! 
//! // IDLE LOOP
//! fn idle(_prio: P0, _ceil: C0) -> ! {
//!     hprintln!("World");
//! 
//!     // Sleep
//!     loop {
//!         rtfm::wfi();
//!     }
//! }
//! 
//! // TASKS
//! tasks!(stm32f30x, {});
//! ```
// Auto-generated. Do not modify.
