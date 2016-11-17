//! `panic!`s send their messages to the ITM
//!
//! See the `itm` example for instruction on how the receive "ITM" messages on
//! the HOST, e.g. your laptop. You should see the following output after
//! running the program below:
//!
//! ``` text
//! $ itmdump /tmp/itm.fifo
//! PANIC at 'Hello, world!', examples/panic.rs:21
//! ```
//!
//! Like with "`std`" panics, these panics also report file and line
//! information.

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m;
extern crate f3;

#[no_mangle]
pub fn main() -> ! {
    panic!("Hello, world!")
}
