//! Board Support Crate for the STM32F3DISCOVERY
//!
//! # Usage
//!
//! Follow `cortex-m-quickstart` [instructions][i] but remove the `memory.x`
//! linker script and the `build.rs` build script file as part of the
//! configuration of the quickstart crate. Additionally, uncomment the "if using
//! ITM" block in the `.gdbinit` file.
//!
//! [i]: https://docs.rs/cortex-m-quickstart/0.2.0/cortex_m_quickstart/
//!
//! # Examples
//!
//! Check the [examples] module.
//!
//! [examples]: ./examples/index.html

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate cast;
pub extern crate stm32f30x;

// For documentation only
pub mod examples;

pub mod led;
pub mod serial;
pub mod timer;

mod frequency;
