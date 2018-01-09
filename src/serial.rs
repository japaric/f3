use core::ptr;

use hal::serial;
use nb;
use stm32f30x::USART1;

use gpio::GPIOA::{PA10, PA9};
use gpio::GPIOB::{PB6, PB7};
use gpio::GPIOC::{PC4, PC5};
use gpio::GPIOE::{PE0, PE1};
use gpio::{AF7, Alternate};
use rcc::APB2;
use time::Bps;

#[derive(Debug)]
pub enum Error {
    Framing,
    Noise,
    Overrun,
    Parity,
    #[doc(hidden)] _Extensible,
}

pub trait Pins {}

impl Pins for (PA9<Alternate<AF7>>, PA10<Alternate<AF7>>) {}
impl Pins for (PB6<Alternate<AF7>>, PB7<Alternate<AF7>>) {}
impl Pins for (PC4<Alternate<AF7>>, PC5<Alternate<AF7>>) {}
impl Pins for (PE0<Alternate<AF7>>, PE1<Alternate<AF7>>) {}

pub struct Serial<PINS> {
    usart: USART1,
    pins: PINS,
}

impl<PINS> Serial<PINS> {
    pub fn new(usart: USART1, pins: PINS, apb2: &mut APB2, baud_rate: Bps) -> Self
    where
        PINS: Pins,
    {
        // FIXME make configurable
        const FREQUENCY: u32 = 8_000_000;

        // enable or reset USART1
        if apb2.enr().read().usart1en().bit_is_set() {
            apb2.rstr().modify(|_, w| w.usart1rst().set_bit());
            apb2.rstr().modify(|_, w| w.usart1rst().clear_bit());
        } else {
            apb2.enr().modify(|_, w| w.usart1en().enabled());
        }

        // configuration
        usart.cr2.reset();

        // disable hardware flow control
        // TODO enable DMA
        usart.cr3.write(|w| w.rtse().clear_bit().ctse().clear_bit());

        let brr = FREQUENCY / baud_rate.0;
        assert!(brr >= 16, "impossible baud rate");
        usart.brr.write(|w| unsafe { w.bits(brr) });

        // UE: enable USART
        // RE: enable receiver
        // TE: enable transceiver
        usart
            .cr1
            .write(|w| w.ue().set_bit().re().set_bit().te().set_bit());

        Serial { usart, pins }
    }

    pub fn split(self) -> (Tx, Rx) {
        (Tx { _0: () }, Rx { _0: () })
    }

    pub fn free(self) -> (USART1, PINS) {
        (self.usart, self.pins)
    }
}

pub struct Rx {
    _0: (),
}

impl serial::Read<u8> for Rx {
    type Error = Error;

    fn read(&mut self) -> nb::Result<u8, Error> {
        // NOTE(unsafe) atomic read with no side effects
        let isr = unsafe { (*USART1::ptr()).isr.read() };

        Err(if isr.pe().bit_is_set() {
            nb::Error::Other(Error::Parity)
        } else if isr.fe().bit_is_set() {
            nb::Error::Other(Error::Framing)
        } else if isr.nf().bit_is_set() {
            nb::Error::Other(Error::Noise)
        } else if isr.ore().bit_is_set() {
            nb::Error::Other(Error::Overrun)
        } else if isr.rxne().bit_is_set() {
            return Ok(unsafe { ptr::read_volatile(&(*USART1::ptr()).rdr as *const _ as *const _) });
        } else {
            nb::Error::WouldBlock
        })
    }
}

pub struct Tx {
    _0: (),
}

impl serial::Write<u8> for Tx {
    // NOTE(!) See section "29.7 USART interrupts"; the only possible errors during transmission
    // are: clear to send (which is disabled in this case) errors and framing errors (which only
    // occur in SmartCard mode); neither of these apply to our hardware configuration
    type Error = !;

    fn flush(&mut self) -> nb::Result<(), !> {
        // NOTE(unsafe) atomic read with no side effects
        let isr = unsafe { (*USART1::ptr()).isr.read() };

        if isr.tc().bit_is_set() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), !> {
        // NOTE(unsafe) atomic read with no side effects
        let isr = unsafe { (*USART1::ptr()).isr.read() };

        if isr.txe().bit_is_set() {
            // NOTE(unsafe) atomic write to stateless register
            // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
            unsafe { ptr::write_volatile(&(*USART1::ptr()).tdr as *const _ as *mut _, byte) }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}
