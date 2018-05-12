//! Board support crate for the STM32F3DISCOVERY
//!
//! # Usage
//!
//! - Trying out the examples
//!
//! ``` text
//! $ # if you don't have the clone subcommand
//! $ cargo install cargo-clone
//!
//! $ cargo clone f3 --vers 0.6.0
//!
//! # on another terminal
//! $ openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
//!
//! # flash and debug the "Hello, world" example
//! $ cd f3
//! $ rustup target add thumbv7em-none-eabihf
//! $ cargo run --example hello
//! ```
//!
//! You'll need to have both OpenOCD and arm-none-eabi-gcc installed.
//!
//! - Building an application that depends on this crate
//!
//! To build applications (binary crates) using this crate follow [cortex-m-quickstart] instructions
//! and add this crate as a dependency in step number 6 and make sure you enable the "rt" Cargo
//! feature of this crate. Also, instead of step number 4 remove *both* the build.rs and memory.x
//! files.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.3
//!
//! # Examples
//!
//! See the [examples] module.
//!
//! [examples]: examples/index.html

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

pub extern crate l3gd20;
pub extern crate lsm303dlhc;
pub extern crate stm32f30x_hal as hal;

use hal::gpio::gpioa::{PA5, PA6, PA7};
use hal::gpio::gpiob::{PB6, PB7};
use hal::gpio::gpioe::PE3;
use hal::gpio::{AF4, AF5, Output, PushPull};
use hal::i2c::I2c;
use hal::spi::Spi;
use hal::stm32f30x::{I2C1, SPI1};

pub mod examples;
pub mod led;

/// On board L3GD20 connected to the SPI1 bus via the pins PA5, PA6, PA7 and PE3
pub type L3gd20 = l3gd20::L3gd20<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>, PE3<Output<PushPull>>>;

/// On board LSM303DLHC connected to the I2C1 bus via the PB6 and PB7 pins
pub type Lsm303dlhc = lsm303dlhc::Lsm303dlhc<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>;
