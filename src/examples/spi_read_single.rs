// Auto-generated. Do not modify this file! Instead modify examples/spi-read-single.rs
//! SPI - read one of the gyroscope registers
//!
//! ``` rust,no_run
//! #![no_main]
//! #![no_std]
//!
//! #[macro_use]
//! extern crate f3;
//!
//! use f3::peripheral;
//!
//! #[no_mangle]
//! pub fn main() -> ! {
//!     // WHO_AM_I
//!     const REGISTER_ADDRESS: u8 = 0x0f;
//!     // Read the register
//!     const READ: u8 = 1 << 7;
//!
//!     let value = unsafe {
//!         let spi1 = peripheral::spi1_mut();
//!         let gpioe = peripheral::gpioe();
//!
//!         // CS: low
//!         gpioe.bsrr.write(|w| w.br3(true));
//!
//!         while !spi1.sr.read().txe() {}
//!         spi1.dr.write_u8(READ | REGISTER_ADDRESS);
//!         while !spi1.sr.read().txe() {}
//!         spi1.dr.write_u8(0x0);
//!
//!         while !spi1.sr.read().rxne() {}
//!         spi1.dr.read_u8();
//!         while !spi1.sr.read().rxne() {}
//!
//!         // CS: high
//!         gpioe.bsrr.write(|w| w.bs3(true));
//!
//!         spi1.dr.read_u8()
//!     };
//!
//!     iprintln!("0x{:02x} - 0x{:02x}", REGISTER_ADDRESS, value);
//!
//!     loop {}
//! }
//! ```
