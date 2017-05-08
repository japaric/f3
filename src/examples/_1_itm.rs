//! Sends Hello and then World through the first ITM stimulus port
//!
//! You'll need to run `itmdump itm.fifo` (mind the file paths) to receive the
//! message on the host. Read the [`itm`] crate documentation for details.
//!
//! [`itm`]: https://docs.rs/itm/0.1.1/itm/
//!
//! ```
//! 
//! #![feature(const_fn)]
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
//! // RESOURCES
//! peripherals!(stm32f30x, {
//!     ITM: Peripheral {
//!         register_block: Itm,
//!         ceiling: C0,
//!     },
//! });
//! 
//! // INITIALIZATION PHASE
//! fn init(ref priority: P0, threshold: &TMax) {
//!     let itm = ITM.access(priority, threshold);
//! 
//!     iprintln!(&itm.stim[0], "Hello");
//! }
//! 
//! // IDLE LOOP
//! fn idle(ref priority: P0, ref threshold: T0) -> ! {
//!     let itm = ITM.access(priority, threshold);
//! 
//!     iprintln!(&itm.stim[0], "World");
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
