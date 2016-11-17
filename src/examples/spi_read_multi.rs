// Auto-generated. Do not modify this file! Instead modify examples/spi-read-multi.rs
//! SPI - read multiple gyroscope registers in "one shot"
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
//!     // OUT_X_L
//!     const REGISTER_ADDRESS_START: u8 = 0x28;
//!     // Read the register
//!     const READ: u8 = 1 << 7;
//!     // Multiple reading.
//!     const MS: u8 = 1 << 6;
//!
//!     let mut bytes = [0; 6];
//!
//!     unsafe {
//!         let spi1 = peripheral::spi1_mut();
//!         let gpioe = peripheral::gpioe();
//!
//!         // CS: low
//!         gpioe.bsrr.write(|w| w.br3(true));
//!
//!         while !spi1.sr.read().txe() {}
//!         spi1.dr.write_u8(READ | MS | REGISTER_ADDRESS_START);
//!         while !spi1.sr.read().rxne() {}
//!         spi1.dr.read_u8();
//!         for byte in &mut bytes {
//!             while !spi1.sr.read().txe() {}
//!             spi1.dr.write_u8(0x0);
//!             while !spi1.sr.read().rxne() {}
//!             *byte = spi1.dr.read_u8();
//!         }
//!
//!         // CS: high
//!         gpioe.bsrr.write(|w| w.bs3(true));
//!     }
//!
//!     for (byte, i) in bytes.iter().zip(0..) {
//!         iprintln!("0x{:02x} - 0x{:02x}", REGISTER_ADDRESS_START + i, byte);
//!     }
//!
//!     loop {}
//! }
//! ```
