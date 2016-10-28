//! SPI
//!
//! - Master Input (`MISO`) pin - `PA6`
//! - Master Output (`MOSI`) pin - `PA7`
//! - Serial Clock (`SCK`) pin - `PA5`

use core::mem;

use cortex_m::interrupt::Mutex;
use futuro::prelude::{Async, Future};

use peripheral;

/// SPI
pub struct Spi {
    _0: (),
}

impl Spi {
    /// Initializes the SPI peripheral
    ///
    /// NOTE There can only be an instance of this peripheral. If you call this
    /// constructor a second time, it will return `None`.
    pub fn new() -> Option<Spi> {
        static YIELDED: Mutex<bool> = Mutex::new(false);

        YIELDED.lock(|yielded| {
            if *yielded {
                None
            } else {
                *yielded = true;

                let gpioa = unsafe { peripheral::gpioa_mut() };
                let rcc = unsafe { peripheral::rcc_mut() };
                let spi1 = unsafe { peripheral::spi1_mut() };

                // RCC: enable GPIOA and SPI1
                rcc.ahbenr.modify(|_, w| w.iopaen(true));
                rcc.apb2enr.modify(|_, w| w.spi1en(true));

                // GPIOA: configure PA5, PA6 and PA7 for SPI use
                // AFRL5 = 5 (SPI1_SCK)
                // AFRL6 = 5 (SPI1_MISO)
                // AFRL7 = 5 (SPI1_MOSI)
                // MODER* = 0b10 (Alternate function)
                gpioa.afrl.modify(|_, w| w.afrl5(5).afrl6(5).afrl7(5));
                gpioa.moder
                    .modify(|_, w| w.moder5(0b10).moder6(0b10).moder7(0b10));

                // SPI1: configuration
                // BIDIMODE + RXONLY: full-duplex (2 unidirectional lines)
                // SSM: enable software slave management
                // SSI: set NSS high
                // LSBFIRST: send the MSB first
                // SPE: enable the peripheral
                // BR: f_PCLK / 8 = 8 MHz / 8 = 1 MHz
                // MSTR: Master configuration
                // CPOL: SCK is 0 when idle (!)
                // CPHA: capture data on the first clock transition
                spi1.cr1.write(|w| {
                    w.bidimode(false)
                        .rxonly(false)
                        .ssm(true)
                        .ssi(true)
                        .lsbfirst(false)
                        .br(0b010)
                        .mstr(true)
                        .cpol(false)
                        .cpha(false)
                });

                // FRXTH: RXNE threshold is 8-bit
                // DS: 8-bit data
                // SSOE: disable output on the NSS pin
                spi1.cr2.write(|w| {
                    w.frxth(true).ds(0b0111).frxth(true).ssoe(false)
                });

                // Enable the peripheral
                spi1.cr1.modify(|_, w| w.spe(true));

                Some(Spi { _0: () })
            }
        })
    }

    /// Sends one `byte` to the slave and returns the byte that the slave
    /// returns
    pub fn transfer(self, byte: u8) -> Transfer {
        Transfer {
            byte: Some(byte),
            spi: Some(self),
        }
    }

    /// Sends `bytes` to the slave and returns the bytes that the slave returned
    pub fn transfer_all<B>(self, bytes: B) -> TransferAll<B>
        where B: AsMut<[u8]> + AsRef<[u8]>
    {
        TransferAll {
            rpos: 0,
            state: Some((self, bytes)),
            wpos: 0,
        }
    }
}

/// Future returned by [Spi::transfer](struct.Spi.html#method.transfer)
#[must_use = "futures do nothing unless polled"]
pub struct Transfer {
    byte: Option<u8>,
    spi: Option<Spi>,
}

impl Future for Transfer {
    type Item = (Spi, u8);

    fn poll(&mut self) -> Async<Self::Item> {
        let spi = mem::replace(&mut self.spi, None)
            .expect("`transfer` can't be polled twice");

        let spi1 = unsafe { peripheral::spi1_mut() };
        if let Some(byte) = self.byte {
            if spi1.sr.read().txe() {
                spi1.dr.write_u8(byte);
                self.byte = None;
            } else {
                self.spi = Some(spi);
                return Async::NotReady;
            }
        }

        if spi1.sr.read().rxne() {
            Async::Ready((spi, spi1.dr.read_u8()))
        } else {
            self.spi = Some(spi);
            Async::NotReady
        }
    }
}

/// Future returned by [Spi::transfer_all](struct.Spi.html#method.transfer_all)
#[must_use = "futures do nothing unless polled"]
pub struct TransferAll<B> {
    rpos: u8,
    state: Option<(Spi, B)>,
    wpos: u8,
}

impl<B> Future for TransferAll<B>
    where B: AsMut<[u8]> + AsRef<[u8]>
{
    type Item = (Spi, B);

    fn poll(&mut self) -> Async<Self::Item> {
        let (spi, mut buffer) = mem::replace(&mut self.state, None)
            .expect("`transfer_all` can't be polled twice");

        let n = buffer.as_ref().len() as u8;

        let spi1 = unsafe { peripheral::spi1_mut() };
        if self.wpos < n {
            while spi1.sr.read().txe() {
                if let Some(byte) = buffer.as_ref()
                    .get(usize::from(self.wpos)) {
                    spi1.dr.write_u8(*byte);
                    self.wpos += 1;
                } else {
                    break;
                }

                let mut ready = false;
                if spi1.sr.read().rxne() {
                    if let Some(slot) = buffer.as_mut()
                        .get_mut(usize::from(self.rpos)) {
                        *slot = spi1.dr.read_u8();
                        self.rpos += 1;
                        ready = self.rpos == n;
                    }

                    if ready {
                        return Async::Ready((spi, buffer));
                    }
                }
            }
        }

        while spi1.sr.read().rxne() {
            let mut ready = false;
            if let Some(slot) = buffer.as_mut()
                .get_mut(usize::from(self.rpos)) {
                *slot = spi1.dr.read_u8();
                self.rpos += 1;
                ready = self.rpos == n;
            }

            if ready {
                return Async::Ready((spi, buffer));
            }
        }

        self.state = Some((spi, buffer));
        Async::NotReady
    }
}
