//! Interfacing the on-board LSM303DLHC (accelerometer + compass)
//!
//! ```
//! #![deny(unsafe_code)]
//! #![deny(warnings)]
//! #![no_std]
//! #![no_main]
//! 
//! extern crate panic_semihosting;
//! 
//! use cortex_m::asm;
//! use cortex_m_rt::entry;
//! use f3::{
//!     hal::{i2c::I2c, prelude::*, stm32f30x},
//!     Lsm303dlhc,
//! };
//! 
//! #[entry]
//! fn main() -> ! {
//!     let p = stm32f30x::Peripherals::take().unwrap();
//! 
//!     let mut flash = p.FLASH.constrain();
//!     let mut rcc = p.RCC.constrain();
//! 
//!     // TRY the other clock configuration
//!     let clocks = rcc.cfgr.freeze(&mut flash.acr);
//!     // let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze(&mut flash.acr);
//! 
//!     // The `Lsm303dlhc` abstraction exposed by the `f3` crate requires a specific pin configuration
//!     // to be used and won't accept any configuration other than the one used here. Trying to use a
//!     // different pin configuration will result in a compiler error.
//!     let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
//!     let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//!     let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
//! 
//!     let i2c = I2c::i2c1(p.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);
//! 
//!     let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();
//! 
//!     let _accel = lsm303dlhc.accel().unwrap();
//!     let _mag = lsm303dlhc.mag().unwrap();
//!     let _temp = lsm303dlhc.temp().unwrap();
//! 
//!     // when you reach this breakpoint you'll be able to inspect the variables `_accel`, `_mag` and
//!     // `_temp` which contain the accelerometer, compass (magnetometer) and temperature sensor
//!     // readings
//!     asm::bkpt();
//! 
//!     loop {}
//! }
//! ```
// Auto-generated. Do not modify.
