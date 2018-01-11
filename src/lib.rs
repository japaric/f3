//! Board support crate for the STM32F3DISCOVERY

#![no_std]

pub extern crate l3gd20;
pub extern crate lsm303dlhc;
pub extern crate stm32f30x_hal as hal;

use hal::i2c::I2c;
use hal::spi::Spi;
use hal::stm32f30x::{I2C1, SPI1};
use hal::gpio::{AF4, AF5, Output, PushPull};
use hal::gpio::GPIOA::{PA5, PA6, PA7};
use hal::gpio::GPIOB::{PB6, PB7};
use hal::gpio::GPIOE::PE3;

pub mod led;

pub type L3gd20 = l3gd20::L3gd20<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>, PE3<Output<PushPull>>>;

pub type Lsm303dlhc = lsm303dlhc::Lsm303dlhc<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>;
