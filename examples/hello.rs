//! Prints "Hello, world" on the OpenOCD console
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as semihosting;
extern crate f3;
extern crate panic_semihosting;

use core::fmt::Write;

use rt::ExceptionFrame;
use semihosting::hio;

entry!(main);

fn main() -> ! {
    writeln!(hio::hstdout().unwrap(), "Hello, world!").unwrap();

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
