//! Sends "Hello, world!" through the first ITM stimulus port
//!
//! To receive the message on the host you'll have to do three things:
//!
//! You'll need to uncomment lines 8 and 15 of the `.gdbinit` file.
//!
//! You'll also need to connect the SWO pin of the on-board SWD programmer to pin PB3 as shown
//! [here](https://japaric.github.io/discovery/06-hello-world/README.html).
//!
//! Finally, you'll need to run `itmdump itm.fifo` (mind the file paths) to receive the message.
//! Read the documentation of the [`itm`] crate, which provides the `itmdump` tool,  for details.
//!
//! [`itm`]: https://docs.rs/itm/0.2.0/itm/
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_main]
//! #![no_std]
//! 
//! #[macro_use]
//! extern crate cortex_m;
//! extern crate f3;
//! extern crate panic_semihosting;
//! #[macro_use(entry, exception)]
//! extern crate cortex_m_rt as rt;
//! 
//! use cortex_m::asm;
//! use rt::ExceptionFrame;
//! 
//! entry!(main);
//! 
//! fn main() -> ! {
//!     let p = cortex_m::Peripherals::take().unwrap();
//!     let mut itm = p.ITM;
//! 
//!     iprintln!(&mut itm.stim[0], "Hello, world!");
//! 
//!     asm::bkpt();
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
