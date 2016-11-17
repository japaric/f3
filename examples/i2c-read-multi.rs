//! I2C - read several magnetometer registers in "one shot"

#![feature(asm)]
#![no_main]
#![no_std]

#[macro_use]
extern crate f3;

use f3::peripheral;

#[no_mangle]
pub fn main() -> ! {
    // Magnetometer
    const SLAVE_ADDRESS: u8 = 0b001_1110;
    // CRA_REG_M
    const START_REGISTER_ADDRESS: u8 = 0x00;

    let mut bytes = [0; 3];

    unsafe {
        let i2c1 = peripheral::i2c1_mut();

        // Configure to send 1 byte to the magnetometer
        // Send START
        i2c1.cr2.modify(|_, w| {
            w.sadd1(SLAVE_ADDRESS).rd_wrn(false).nbytes(1).start(true)
        });

        // Wait until we are allowed to send data (START has been ACK)
        while !i2c1.isr.read().txis() {}

        i2c1.txdr.write(|w| w.txdata(START_REGISTER_ADDRESS));

        // Wait until the transmission is complete
        while !i2c1.isr.read().tc() {}

        // Configure to receive 3 byte from the magnetometer
        // (re)START
        i2c1.cr2.modify(|_, w| {
            w.rd_wrn(true).nbytes(bytes.len() as u8).start(true).autoend(true)
        });

        for byte in &mut bytes {
            // Wait until we have received something
            while !i2c1.isr.read().rxne() {}

            *byte = i2c1.rxdr.read().rxdata();
        }
        // STOP (automatic)
    };

    for (byte, i) in bytes.iter().zip(0..) {
        iprintln!("0x{:02x} - 0x{:02x}", START_REGISTER_ADDRESS + i, byte);
    }

    loop {}
}
