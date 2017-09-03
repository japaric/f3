//! Serial interface
//!
//! You can use the `Serial` interface with these USART instances
//!
//! # USART1
//!
//! - TX = PA9
//! - RX = PA10
//! - Interrupt = USART1

use core::any::{Any, TypeId};
use core::marker::Unsize;
use core::ops::Deref;
use core::ptr;

use cast::u16;
use hal;
use nb;
use static_ref::Static;
use stm32f30x::{gpioa, DMA1, USART1, usart1, GPIOA,
                  RCC};

use dma::{self, Buffer, Dma1Channel4, Dma1Channel5};

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, nb::Error<Error>>;

/// IMPLEMENTATION DETAIL
pub unsafe trait Usart: Deref<Target = usart1::RegisterBlock> {
    /// IMPLEMENTATION DETAIL
    type GPIO: Deref<Target = gpioa::RegisterBlock>;
    /// IMPLEMENTATION DETAIL
    type Ticks: Into<u32>;
}

unsafe impl Usart for USART1 {
    type GPIO = GPIOA;
    type Ticks = ::apb2::Ticks;
}

/// An error
#[derive(Debug)]
pub enum Error {
    /// De-synchronization, excessive noise or a break character detected
    Framing,
    /// Noise detected in the received frame
    Noise,
    /// RX buffer overrun
    Overrun,
    #[doc(hidden)]
    _Extensible,
}

/// Interrupt event
pub enum Event {
    /// RX buffer Not Empty (new data available)
    Rxne,
    /// Transmission Complete
    Tc,
    /// TX buffer Empty (more data can be send)
    Txe,
}

/// Serial interface
///
/// # Interrupts
///
/// - RXNE
pub struct Serial<'a, U>(pub &'a U)
where
    U: Any + Usart;

impl<'a, U> Clone for Serial<'a, U>
where
    U: Any + Usart,
{
    fn clone(&self) -> Self {
        *self
    }
}

impl<'a, U> Copy for Serial<'a, U>
where
    U: Any + Usart,
{
}

impl<'a, U> Serial<'a, U>
where
    U: Any + Usart,
{
    /// Initializes the serial interface with a baud rate of `baut_rate` bits
    /// per second
    ///
    /// The serial interface will be configured to use 8 bits of data, 1 stop
    /// bit, no hardware control and to omit parity checking
    pub fn init<B>(
        &self,
        baud_rate: B,
        dma1: Option<&DMA1>,
        gpio: &U::GPIO,
        rcc: &RCC,
    ) where
        B: Into<U::Ticks>,
    {
        self._init(baud_rate.into(), dma1, gpio, rcc)
    }

    fn _init(
        &self,
        baud_rate: U::Ticks,
        dma1: Option<&DMA1>,
        gpio: &U::GPIO,
        rcc: &RCC,
    ) {
        let usart = self.0;

        // power up peripherals
        if dma1.is_some() {
            rcc.ahbenr.modify(|_, w| w.dmaen().enabled());
        }
        if usart.get_type_id() == TypeId::of::<USART1>() {
            rcc.apb2enr.modify(|_, w| {
                w.usart1en().enabled()
            });
        }

        rcc.ahbenr.modify(|_, w| w.iopaen().set_bit());

        if usart.get_type_id() == TypeId::of::<USART1>() {
            // PA9. = TX, PA10 = RX
            gpio.afrh.modify(|_, w| {
                unsafe {
                    w.afrh9().bits(7).afrh10().bits(7)
                }
            });
            gpio.moder.modify(|_, w| {
                w.moder9().alternate()
                    .moder10().alternate()
            });
        }

        if let Some(dma1) = dma1 {
            if usart.get_type_id() == TypeId::of::<USART1>() {
                // TX DMA transfer
                // mem2mem: Memory to memory mode disabled
                // pl: Medium priority
                // msize: Memory size = 8 bits
                // psize: Peripheral size = 8 bits
                // minc: Memory increment mode enabled
                // pinc: Peripheral increment mode disabled
                // circ: Circular mode disabled
                // dir: Transfer from memory to peripheral
                // tceie: Transfer complete interrupt enabled
                // en: Disabled
                dma1.ccr4.write(|w| unsafe {
                    w.mem2mem()
                        .clear_bit()
                        .pl()
                        .bits(0b01)
                        .msize()
                        .bits(0b00)
                        .psize()
                        .bits(0b00)
                        .minc()
                        .set_bit()
                        .circ()
                        .clear_bit()
                        .pinc()
                        .clear_bit()
                        .dir()
                        .set_bit()
                        .tcie()
                        .set_bit()
                        .en()
                        .clear_bit()
                });

                // RX DMA transfer
                // mem2mem: Memory to memory mode disabled
                // pl: Medium priority
                // msize: Memory size = 8 bits
                // psize: Peripheral size = 8 bits
                // minc: Memory increment mode enabled
                // pinc: Peripheral increment mode disabled
                // circ: Circular mode disabled
                // dir: Transfer from peripheral to memory
                // tceie: Transfer complete interrupt enabled
                // en: Disabled
                dma1.ccr5.write(|w| unsafe {
                    w.mem2mem()
                        .clear_bit()
                        .pl()
                        .bits(0b01)
                        .msize()
                        .bits(0b00)
                        .psize()
                        .bits(0b00)
                        .minc()
                        .set_bit()
                        .circ()
                        .clear_bit()
                        .pinc()
                        .clear_bit()
                        .dir()
                        .clear_bit()
                        .tcie()
                        .set_bit()
                        .en()
                        .clear_bit()
                });
            }
        }

        // 8N1
        usart.cr2.write(|w| unsafe { w.stop().bits(0b00) });

        // baud rate
        let brr = baud_rate.into();
        assert!(brr >= 16, "impossible baud rate");
        usart.brr.write(|w| unsafe { w.bits(brr) });

        // disable hardware flow control
        // enable DMA TX and RX transfers
        usart.cr3.write(|w| {
            w.rtse()
                .clear_bit()
                .ctse()
                .clear_bit()
                .dmat()
                .set_bit()
                .dmar()
                .set_bit()
        });

        // enable TX, RX; disable parity checking
        usart.cr1.write(|w| {
            w.ue()
                .set_bit()
                .re()
                .set_bit()
                .te()
                .set_bit()
                .m()
                .clear_bit()
                .over8()
                .clear_bit()
                .pce()
                .clear_bit()
                .rxneie()
                .clear_bit()
        });
    }

    /// Starts listening for an interrupt `event`
    pub fn listen(&self, event: Event) {
        let usart = self.0;

        match event {
            Event::Rxne => usart.cr1.modify(|_, w| w.rxneie().set_bit()),
            Event::Tc => usart.cr1.modify(|_, w| w.tcie().set_bit()),
            Event::Txe => usart.cr1.modify(|_, w| w.txeie().set_bit()),
        }
    }

    /// Stops listening for an interrupt `event`
    pub fn unlisten(&self, event: Event) {
        let usart = self.0;

        match event {
            Event::Rxne => usart.cr1.modify(|_, w| w.rxneie().clear_bit()),
            Event::Tc => usart.cr1.modify(|_, w| w.tcie().clear_bit()),
            Event::Txe => usart.cr1.modify(|_, w| w.txeie().clear_bit()),
        }
    }
}

impl<'a, U> hal::serial::Read<u8> for Serial<'a, U>
where
    U: Any + Usart,
{
    type Error = Error;

    fn read(&self) -> Result<u8> {
        let usart1 = self.0;
        let sr = usart1.isr.read();

        if sr.ore().bit_is_set() {
            Err(nb::Error::Other(Error::Overrun))
        } else if sr.nf().bit_is_set() {
            Err(nb::Error::Other(Error::Noise))
        } else if sr.fe().bit_is_set() {
            Err(nb::Error::Other(Error::Framing))
        } else if sr.rxne().bit_is_set() {
            // NOTE(read_volatile) the register is 9 bits big but we'll only
            // work with the first 8 bits
            Ok(unsafe {
                ptr::read_volatile(&usart1.rdr as *const _ as *const u8)
            })
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<'a, U> hal::serial::Write<u8> for Serial<'a, U>
where
    U: Any + Usart,
{
    type Error = Error;

    fn write(&self, byte: u8) -> Result<()> {
        let usart1 = self.0;
        let sr = usart1.isr.read();

        if sr.ore().bit_is_set() {
            Err(nb::Error::Other(Error::Overrun))
        } else if sr.nf().bit_is_set() {
            Err(nb::Error::Other(Error::Noise))
        } else if sr.fe().bit_is_set() {
            Err(nb::Error::Other(Error::Framing))
        } else if sr.txe().bit_is_set() {
            // NOTE(write_volatile) see NOTE in the `read` method
            unsafe {
                ptr::write_volatile(&usart1.tdr as *const _ as *mut u8, byte)
            }
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<'a> Serial<'a, USART1> {
    /// Starts a DMA transfer to receive serial data into a `buffer`
    ///
    /// This will mutably lock the `buffer` preventing borrowing its contents
    /// The `buffer` can be `release`d after the DMA transfer finishes
    // TODO support circular mode + half transfer interrupt as a double
    // buffering mode
    pub fn read_exact<B>(
        &self,
        dma1: &DMA1,
        buffer: &Static<Buffer<B, Dma1Channel5>>,
    ) -> ::core::result::Result<(), dma::Error>
    where
        B: Unsize<[u8]>,
    {
        let usart1 = self.0;

        if dma1.ccr5.read().en().bit_is_set() {
            return Err(dma::Error::InUse);
        }

        let buffer: &mut [u8] = buffer.lock_mut();

        dma1.cndtr5
            .write(|w| unsafe { w.ndt().bits(u16(buffer.len()).unwrap()) });
        dma1.cpar5
            .write(|w| unsafe { w.bits(&usart1.rdr as *const _ as u32) });
        dma1.cmar5
            .write(|w| unsafe { w.bits(buffer.as_ptr() as u32) });
        dma1.ccr5.modify(|_, w| w.en().set_bit());

        Ok(())
    }

    /// Starts a DMA transfer to send `buffer` through this serial port
    ///
    /// This will immutably lock the `buffer` preventing mutably borrowing its
    /// contents. The `buffer` can be `release`d after the DMA transfer finishes
    pub fn write_all<B>(
        &self,
        dma1: &DMA1,
        buffer: &Static<Buffer<B, Dma1Channel4>>,
    ) -> ::core::result::Result<(), dma::Error>
    where
        B: Unsize<[u8]>,
    {
        let usart1 = self.0;

        if dma1.ccr4.read().en().bit_is_set() {
            return Err(dma::Error::InUse);
        }

        let buffer: &[u8] = buffer.lock();

        dma1.cndtr4
            .write(|w| unsafe { w.ndt().bits(u16(buffer.len()).unwrap()) });
        dma1.cpar4
            .write(|w| unsafe { w.bits(&usart1.tdr as *const _ as u32) });
        dma1.cmar4
            .write(|w| unsafe { w.bits(buffer.as_ptr() as u32) });
        dma1.ccr4.modify(|_, w| w.en().set_bit());

        Ok(())
    }
}
