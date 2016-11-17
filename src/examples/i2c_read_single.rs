// Auto-generated. Do not modify this file! Instead modify examples/i2c-read-single.rs
//! I2C - read one of the accelerometer registers
//!
//! ``` rust,no_run
//! #![feature(asm)]
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
//!     // Accelerometer
//!     const SLAVE_ADDRESS: u8 = 0b0011001;
//!     // CTRL_REG1_A
//!     const REGISTER_ADDRESS: u8 = 0x20;
//!
//!     let value = unsafe {
//!         let i2c1 = peripheral::i2c1_mut();
//!
//!         // Configure to send 1 byte to the accelerometer
//!         // Send START
//!         i2c1.cr2.modify(|_, w| {
//!             w.sadd1(SLAVE_ADDRESS).rd_wrn(false).nbytes(1).start(true)
//!         });
//!
//!         // Wait until we are allowed to send data (START has been ACK)
//!         while !i2c1.isr.read().txis() {}
//!
//!         // Send the register address
//!         i2c1.txdr.write(|w| w.txdata(REGISTER_ADDRESS));
//!
//!         // Wait until the transmission is complete
//!         while !i2c1.isr.read().tc() {}
//!
//!         // Configure to receive 1 byte from the accelerometer
//!         // (re)START
//!         i2c1.cr2.modify(|_, w| w.rd_wrn(true).start(true).autoend(true));
//!
//!         // Wait until we have received something
//!         while !i2c1.isr.read().rxne() {}
//!         // STOP (automatic)
//!
//!         i2c1.rxdr.read().rxdata()
//!     };
//!
//!     iprintln!("0x{:02x} - 0x{:02x}", REGISTER_ADDRESS, value);
//!
//!     loop {}
//! }
//! ```
