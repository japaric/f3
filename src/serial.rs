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
        // Transmit data register empty
        // 0: data is not transferred to the shift register
        // 1: data is transferred to the shift register
        const TXE: u32 = 1 << 7;

        let usart1 = peripheral::usart1();

        for byte in s.as_bytes().iter().cloned() {
            while usart1.isr.read() & TXE == 0 {}
            usart1.tdr.write_u8(byte);
        }

        Ok(())
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
    let apb2enr = rcc.apb2enr.read();
    rcc.apb2enr.write({
        const USART1EN: u32 = 1 << 14;

        apb2enr | USART1EN
    });

    let ahbenr = rcc.ahbenr.read();
    rcc.ahbenr.write({
        const IOPAEN: u32 = 1 << 17;

        ahbenr | IOPAEN
    });

    // GPIO: configure PA9 as TX and PA10 as RX
    let afrh = gpioa.afrh.read();
    gpioa.afrh.write({
        // AF7 = USART1_TX
        const AFR9: u32 = 7 << 4;
        const AFR9_MASK: u32 = 0b1111 << 4;
        // AF7 = USART1_RX
        const AFR10: u32 = 7 << 8;
        const AFR10_MASK: u32 = 0b1111 << 8;

        (((afrh & !AFR9_MASK) | AFR9) & !AFR10_MASK) | AFR10
    });
    let moder = gpioa.moder.read();
    gpioa.moder.write({
        // PA9 = Alternate mode
        const MODER9: u32 = 0b10 << 18;
        const MODER9_MASK: u32 = 0b11 << 18;
        // PA10 = Alternate mode
        const MODER10: u32 = 0b10 << 20;
        const MODER10_MASK: u32 = 0b10 << 20;

        (((moder & !MODER9_MASK) | MODER9) & !MODER10_MASK) | MODER10
    });

    // USART1: 115200 - 8N1
    usart1.cr2.write({
        // 1 stop bit
        const STOP: u32 = 00 << 12;

        STOP
    });

    usart1.cr3.write({
        // Enable DMA for the transmitter
        // const DMAT: u32 = 1 << 7;
        // Disable hardware flow control
        const RTSE: u32 = 0 << 8;
        // Disable hardware flow control
        const CTSE: u32 = 0 << 9;

        RTSE | CTSE
    });

    const BAUD_RATE: u32 = 115200;
    usart1.brr.write((::APB2_CLOCK / BAUD_RATE) as u16);

    usart1.cr1.write({
        // Enable USART
        const UE: u32 = 1 << 0;
        // Enable the receiver
        const RE: u32 = 1 << 2;
        // Enable the transmitter
        const TE: u32 = 1 << 3;
        // No parity check
        const PCE: u32 = 0 << 10;
        // Oversampling by 16 -- to set the baud rate
        const OVER8: u32 = 0 << 15;
        // Start bit, 8 data bits, n stop bits
        const M: u32 = 0 << 16 | 0 << 28;

        UE | RE | TE | PCE | OVER8 | M
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
