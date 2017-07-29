//! Serial interface
//!
//! - TX - PA9
//! - RX - PA10

use core::ptr;

use cast::{u16, u8};
use stm32f30x::{GPIOA, RCC, USART1};

use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// Serial interface
///
/// # Interrupts
///
/// - `Usart1Exti25` - RXNE (RX buffer not empty)
#[derive(Clone, Copy)]
pub struct Serial<'a>(pub &'a USART1);

impl<'a> Serial<'a> {
    /// Initializes the serial interface with a baud rate of `baut_rate` bits
    /// per second
    pub fn init(self, gpioa: &GPIOA, rcc: &RCC, baud_rate: u32) {
        let usart1 = self.0;

        // Power up the peripherals
        rcc.apb2enr.modify(|_, w| w.usart1en().enabled());
        rcc.ahbenr.modify(|_, w| w.iopaen().enabled());

        // Configure PA9 as TX and PA10 as RX
        gpioa.afrh.modify(|_, w| unsafe {
            w.afrh9().bits(7).afrh10().bits(7)
        });
        gpioa.moder.modify(|_, w| {
            w.moder9().alternate().moder10().alternate()
        });

        // 8 data bits, 0 stop bits
        usart1.cr2.write(|w| unsafe { w.stop().bits(0b00) });

        // Disable hardware flow control
        usart1.cr3.write(|w| w.rtse().bit(false).ctse().bit(false));

        // set baud rate
        let brr = u16(frequency::APB2 / baud_rate).unwrap();
        let fraction = u8(brr & 0b1111).unwrap();
        let mantissa = brr >> 4;
        usart1.brr.write(|w| unsafe {
            w.div_fraction().bits(fraction).div_mantissa().bits(
                mantissa,
            )
        });

        // enable peripheral, transmitter, receiver
        // enable RXNE event
        usart1.cr1.write(|w| {
            w.ue()
                .bit(true)
                .re()
                .bit(true)
                .te()
                .bit(true)
                .pce()
                .bit(false)
                .over8()
                .bit(false)
                .rxneie()
                .bit(true)
        });
    }

    /// Reads a byte from the RX buffer
    ///
    /// Returns `None` if the buffer is empty
    pub fn read(self) -> Result<u8> {
        let usart1 = self.0;

        if usart1.isr.read().rxne().bit_is_set() {
            // NOTE(read_volatile) the register is 9 bits big but we'll only
            // work with the first 8 bits
            Ok(unsafe {
                ptr::read_volatile(&usart1.rdr as *const _ as *const u8)
            })
        } else {
            Err(Error { _0: () })
        }
    }

    /// Writes byte into the TX buffer
    ///
    /// Returns `Err` if the buffer is already full
    pub fn write(self, byte: u8) -> Result<()> {
        let usart1 = self.0;

        if usart1.isr.read().txe().bit_is_set() {
            unsafe { ptr::write_volatile(&usart1.tdr as *const _ as *mut u8, byte) }
            Ok(())
        } else {
            Err(Error { _0: () })
        }
    }
}
