//! A crate to play with the [STM32F3DISCOVERY]
//!
//! [STM32F3DISCOVERY]: http://www.st.com/en/evaluation-tools/stm32f3discovery.html
//!
//! (What? You don't have one? How come? They are awesome and cheap ($15 +
//! shipping))
//!
//! (No, I'm not associated to STM32. I just like this board in particular.)
//!
//! # Features
//!
//! - High-level API over LEDs, sensors, timers, etc.
//!
//! - An `iprint!` family of macros that sink their output to the ITM
//!   (Instrumentation Trace Macrocell) so you send data to the host over the
//!   same USB cable that you are using to debug your device.
//!
//! - By default, `panic!`s also sink their messages to the ITM
//!
//! - An `uprint!` family of macros that send their messages through the Serial
//!   interface.
//!
//! - By default, an informative exception handler that tells you what went
//!   wrong.
//!
//! - By default, everything (LEDs, sensors, etc) is initialized before the user
//!   entry point, `main`. So everything Just Works out of the box.
//!
//! - Plenty of examples
//!
//! # Requirements and starter code
//!
//! Today, you need these 6 things, one of them optional, but hopefully you
//! won't need 3 of them in the future:
//!
//! - Nightly Rust compiler newer than 2016-10-05: `rustup default nightly`
//!
//! - [Xargo](https://crates.io/crates/xargo) version 0.1.13 or newer. (After
//!   [rust-lang/rfcs#1133](https://github.com/rust-lang/rfcs/pull/1133) gets
//!   accepted and implemented you won't need Xargo anymore)
//!
//! - A binary Cargo project that depends on this crate.
//!
//! ``` text
//! $ cargo new --bin foo && cd $_
//! $ edit Cargo.toml && tail -n2 $_
//! [dependencies]
//! f3 = "0.1.0"
//! ```
//!
//! - Optionally, you can also set `profile.release.lto = true` for even smaller
//!   binaries.
//!
//! ``` text
//! $ edit Cargo.toml && tail -n2 $_
//! [profile.release]
//! lto = true
//! ```
//!
//! - This `.cargo/config` in the root of your Cargo project. (If Cargo build
//!   scripts ever gain a feature to pass arbitrary arguments to the linker then
//!   you won't *need* this. Setting `build.target` though always improves
//!   ergonomics)
//!
//! ``` text
//! $ cat .cargo/config
//! [build]
//! target = "thumbv7em-none-eabihf"
//!
//! [target.thumbv7em-none-eabihf]
//! rustflags = [
//!     "-C",
//!     "link-arg=-Tstm32f3discovery.ld",
//!     "-C",
//!     "link-arg=-nostartfiles",
//! ]
//! ```
//!
//! - And this starter code:
//!
//! ``` text
//! $ cat src/main.rs
//! #![no_main]
//! #![no_std]
//!
//! extern crate f3;
//!
//! #[export_name = "main"]
//! pub fn main() -> ! {
//!     // Your code goes here!
//!
//!     loop {}
//! }
//! ```
//!
//! With all that in place, you can finally build the project using Xargo:
//!
//! ``` text
//! $ xargo build [--target thumbv7em-none-eabihf] [--release]
//! ```
//!
//! Check out my [Copper] book for instructions on how to Flash and Debug this
//! program!
//!
//! [Copper]: https://japaric.github.io/copper/
//!
//! Also, check out my [Discovery] book for an introductory course in
//! microcontrollers based on the STM32F3DISCOVERY.
//!
//! [Discovery]: https://japaric.github.io/discovery
//!
//! # Overriding default behaviors
//!
//! Cargo features can be used to disable/override some default behaviors:
//!
//! - "default-exception-handler". If disabled, the default exception handler
//!   can be overridden using the `_default_exception_handler` symbol
//!   (`extern "C" fn`).
//!
//! - "default-init". If disabled, the pre-main initialization routine can be
//!   overridden via the `_init` symbol (`fn`).
//!
//! - "default-panic-fmt". If disabled, you can override the default behavior of
//!   `panic!` using the `panic_fmt` lang item (`extern "C" fn`) .
//!
//! - "interrupts". If disabled, interrupts can't be used (the handlers can't be
//!   "installed"/overriden). This saves some space in Flash memory.
//!
//! - "static-ram". If disabled, the RAM initialization routine is not executed
//!    and `static mut` variables can't be used. This saves some space in Flash
//!    memory and makes the boot process slightly faster.
//!
//! # Overriding the interrupt and exception handlers
//!
//! By default, all the interrupts and exceptions are handled using the same
//! "handler" (function). You can override this behavior exposing a "symbol"
//! (`extern "C" fn`) in your crate. For example, the override the TIM7
//! interrupt handler, you would expose the `_tim7` symbol:
//!
//! ```
//! #[export_name = "_tim7"]
//! pub extern "C" fn my_tim7_interrupt_handler() {
//!     // ..
//! }
//! ```
//!
//! For a list of these symbol, check the [exception] and [interrupt] modules.
//! All the overridable symbols start with an underscore (`_`).
//!
//! [exception]: exception/index.html
//! [interrupt]: interrupt/index.html
//!
//! # Examples
//!
//! See the [examples](examples/index.html) module.

#![cfg_attr(all(target_arch = "arm",
                feature = "default-exception-handler"),
            feature(core_intrinsics))]
#![deny(missing_docs)]
#![deny(warnings)]
#![feature(asm)]
#![feature(lang_items)]
#![feature(macro_reexport)]
#![feature(naked_functions)]
#![no_std]

#[macro_use]
#[macro_reexport(bkpt)]
extern crate cortex_m;
extern crate compiler_builtins_snapshot;
#[cfg(feature = "static-ram")]
extern crate r0;
extern crate ref_slice;

pub extern crate stm32f30x_memory_map as peripheral;

#[macro_use]
mod macros;

#[cfg(target_arch = "arm")]
mod lang_items;

pub mod delay;
#[cfg(feature = "examples")]
pub mod examples;
pub mod exception;
pub mod fpu;
pub mod interrupt;
pub mod itm;
pub mod l3gd20;
pub mod led;
pub mod lsm303dlhc;
pub mod serial;
pub mod time;

/// Three `i16` integers packed in a struct
#[derive(Debug)]
pub struct I16x3 {
    /// X component
    pub x: i16,
    /// Y component
    pub y: i16,
    /// Z component
    pub z: i16,
}

// Default initialization routine
#[cfg(feature = "default-init")]
#[doc(hidden)]
pub unsafe fn _init() {
    delay::init();
    fpu::init();
    itm::init();
    l3gd20::init();
    led::init();
    lsm303dlhc::init();
    serial::init();
    time::init();
}

// Hz
const APB1_CLOCK: u32 = 8_000_000;
const APB2_CLOCK: u32 = 8_000_000;
