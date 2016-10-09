//! Serial Port communication
//!
//! - Baud rate: `115200`
//! - "Transmit" (`TX`) pin - `PA9`
//! - "Receive" (`RX`) pin - `PA10`
//!
//! # References
//!
//! - RM0316: STM32F303xC [Reference Manual] - Section 29.8 USART Registers
//!
//! [Reference Manual]: http://www.st.com/resource/en/reference_manual/dm00043574.pdf

use core::fmt::{self, Arguments, Write};

use peripheral;

struct Port {}

impl Write for Port {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            let usart1 = peripheral::usart1_mut();

            for byte in s.as_bytes().iter().cloned() {
                while !usart1.isr.read().txe() {}
                usart1.tdr.write(|w| w.tdr(byte as u16));
            }

            Ok(())
        }
    }
}

/// Initializes the necessary stuff to be able to use the Serial Port
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let gpioa = peripheral::gpioa_mut();
    let rcc = peripheral::rcc_mut();
    let usart1 = peripheral::usart1_mut();

    // RCC: Enable USART1 and GPIOC
    rcc.apb2enr.modify(|_, w| w.usart1en(true));
    rcc.ahbenr.modify(|_, w| w.iopaen(true));

    // GPIO: configure PA9 as TX and PA10 as RX
    // AFRH9: USART1_TX
    // AFRH10: USART1_RX
    gpioa.afrh.modify(|_, w| w.afrh9(7).afrh10(7));
    // MODER9: Alternate mode
    // MODER10: Alternate mode
    gpioa.moder.modify(|_, w| w.moder9(0b10).moder10(0b10));

    // USART1: 115200 - 8N1
    usart1.cr2.write(|w| w.stop(0b00));

    // Disable hardware flow control
    usart1.cr3.write(|w| w.rtse(false).ctse(false));

    const BAUD_RATE: u32 = 115200;
    let brr = (::APB2_CLOCK / BAUD_RATE) as u16;
    usart1.brr.write(|w| {
        w.div_fraction((brr & 0b1111) as u8)
            .div_mantissa(brr >> 4)
    });

    // UE: Enable USART
    // RE: Enable the receiver
    // TE: Enable the transmitter
    // PCE: No parity
    // OVER8: Oversampling by 16 -- to set the baud rate
    usart1.cr1.write(|w| {
        w.ue(true)
            .re(true)
            .te(true)
            .pce(false)
            .over8(false)
    });
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {
    Port {}.write_fmt(args).ok();
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    Port {}.write_str(s).ok();
}
