//! Serial
//!
//! - Baud rate: `115200`
//! - "Transmit" (`TX`) pin - `PA9`
//! - "Receive" (`RX`) pin - `PA10`

use core::marker::PhantomData;
use core::mem;

use cortex_m::interrupt::Mutex;
use futuro::prelude::{Async, Future, InfiniteStream};
use peripheral;

/// Serial
pub struct Serial {
    /// Receiver
    pub rx: Rx,
    /// Transmitter
    pub tx: Tx,
}

impl Serial {
    /// Initializes the Serial peripheral
    ///
    /// NOTE There can only be an instance of this peripheral. If you call this
    /// constructor a second time, it will return `None`.
    pub fn new() -> Option<Self> {
        static YIELDED: Mutex<bool> = Mutex::new(false);

        YIELDED.lock(|yielded| {
            if *yielded {
                None
            } else {
                *yielded = true;

                let gpioa = unsafe { peripheral::gpioa_mut() };
                let rcc = unsafe { peripheral::rcc_mut() };
                let usart1 = unsafe { peripheral::usart1_mut() };

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

                Some(Serial {
                    rx: Rx { _0: () },
                    tx: Tx { _0: () },
                })
            }
        })
    }
}

/// Receiver
pub struct Rx {
    _0: (),
}

impl Rx {
    /// Reads an infinite stream of bytes
    pub fn bytes(&mut self) -> Bytes {
        Bytes { _marker: PhantomData }
    }
}

/// Infinite stream returned by [Rx::bytes](struct.Rx.html#method.bytes)
#[must_use = "streams do nothing unless polled"]
pub struct Bytes<'r> {
    _marker: PhantomData<&'r mut Rx>,
}

impl<'r> InfiniteStream for Bytes<'r> {
    type Item = u8;

    fn poll(&mut self) -> Async<u8> {
        let usart1 = peripheral::usart1();

        if usart1.isr.read().rxne() {
            Async::Ready(usart1.rdr.read().rdr() as u8)
        } else {
            Async::NotReady
        }
    }
}

/// Transmitter
pub struct Tx {
    _0: (),
}

impl Tx {
    /// Sends a single `byte`
    pub fn write(self, byte: u8) -> Write {
        Write {
            byte: byte,
            tx: Some(self),
        }
    }

    /// Sends `bytes`
    pub fn writerator<B>(self, bytes: B) -> Writerator<B::IntoIter>
        where B: IntoIterator<Item = u8>
    {
        Writerator {
            bytes: bytes.into_iter(),
            tx: Some(self),
        }
    }
}

/// Future returned by [Tx::write](struct.Tx.html#method.write)
#[must_use = "futures do nothing unless polled"]
pub struct Write {
    tx: Option<Tx>,
    byte: u8,
}

impl Future for Write {
    type Item = Tx;

    fn poll(&mut self) -> Async<Tx> {
        let tx = mem::replace(&mut self.tx, None)
            .expect("`write` can't be polled twice");

        let usart1 = unsafe { peripheral::usart1_mut() };
        if usart1.isr.read().txe() {
            usart1.tdr.write(|w| w.tdr(u16::from(self.byte)));
            Async::Ready(tx)
        } else {
            self.tx = Some(tx);
            Async::NotReady
        }
    }
}

/// Future returned by [Tx::writerator](struct.Tx.html#method.writerator)
#[must_use = "futures do nothing unless polled"]
pub struct Writerator<B> {
    tx: Option<Tx>,
    bytes: B,
}

impl<B> Future for Writerator<B>
    where B: Iterator<Item = u8>
{
    type Item = Tx;

    fn poll(&mut self) -> Async<Tx> {
        let tx = mem::replace(&mut self.tx, None)
            .expect("`write_all` can't be polled twice");

        let usart1 = unsafe { peripheral::usart1_mut() };
        while usart1.isr.read().txe() {
            if let Some(byte) = self.bytes.next() {
                usart1.tdr.write(|w| w.tdr(u16::from(byte)));
            } else {
                return Async::Ready(tx);
            }
        }

        self.tx = Some(tx);
        Async::NotReady
    }
}
