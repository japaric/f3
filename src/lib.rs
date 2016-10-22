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
//! - An `iprint!` family of macros that sink their output to the ITM
//!   (Instrumentation Trace Macrocell) so you send data to the host over the
//!   same USB cable that you are using to debug your device.
//! - By default, `panic!`s also sink their messages to the ITM
//! - By default, an informative exception handler that tells you what went
//!   wrong.
//! - By default, everything (LEDs, sensors, etc) is initialized before the user
//!   entry point, `main`. So everything Just Works out of the box.
//! - Plenty of examples
//!
//! Also, all the "default" behaviors can be overridden:
//!
//! - The default exception handler
//! - The default `panic_fmt` implementation
//! - The default system initialization routine that runs before main.
//!
//! # Requirements and starter code
//!
//! Today, you need these 7 things, one of them optional, but hopefully you
//! won't need 3 of them in the future:
//!
//! - Nightly Rust compiler: `rustup default nightly`
//! - [Xargo](https://crates.io/crates/xargo) version 0.1.12 or newer. (After
//!   [rust-lang/rfcs#1133](https://github.com/rust-lang/rfcs/pull/1133) gets
//!   accepted and implemented you won't need Xargo anymore)
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
//! - This target specification file. (You won't need this after 2016-10-05 as
//!   these targets have already landed
//!   [in the compiler](https://github.com/rust-lang/rust/pull/36874))
//!
//! ``` text
//! $ cat thumbv7em-none-eabihf.json
//! {
//!     "arch": "arm",
//!     "data-layout": "e-m:e-p:32:32-i64:64-v128:64:128-a:0:32-n32-S64",
//!     "executables": true,
//!     "features": "+vfp4,+d16,+fp-only-sp",
//!     "linker": "arm-none-eabi-gcc",
//!     "llvm-target": "thumbv7em-none-eabihf",
//!     "os": "none",
//!     "panic-strategy": "abort",
//!     "relocation-model": "static",
//!     "target-endian": "little",
//!     "target-pointer-width": "32"
//! }
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
//! pub extern "C" fn main() -> ! {
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
//! Check out the [Copper] book for instructions on how to Flash and Debug this
//! program!
//!
//! [Copper]: http://japaric.github.io/copper/
//!
//! # Examples
//!
//! See the [examples](examples/index.html) module.

#![cfg_attr(all(target_arch = "arm",
                feature = "default-exception-handler"),
            feature(core_intrinsics))]
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
extern crate r0;
extern crate volatile_register;

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
pub mod peripheral;
pub mod serial;
pub mod timeit;

#[derive(Debug)]
pub struct I16x3 {
    pub x: i16,
    pub y: i16,
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
    timeit::init();
}

// Hz
const APB1_CLOCK: u32 = 8_000_000;
const APB2_CLOCK: u32 = 8_000_000;
