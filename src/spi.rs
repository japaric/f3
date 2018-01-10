use core::ptr;

use hal::spi::{FullDuplex, Mode, Phase, Polarity};
use nb;
use stm32f30x::{SPI1, SPI2, SPI3};

use gpio::GPIOA::{PA5, PA6, PA7};
use gpio::GPIOB::{PB13, PB14, PB15, PB5};
use gpio::GPIOC::{PC10, PC11, PC12};
use gpio::{AF5, AF6, Alternate};
use rcc::{APB1, APB2, Clocks};
use time::Hertz;

#[derive(Debug)]
pub enum Error {
    /// Overrun occurred
    Overrun,
    /// Mode fault occurred
    ModeFault,
    /// CRC error
    Crc,
    #[doc(hidden)] _Extensible,
}

pub unsafe trait Sck<SPI> {}
pub unsafe trait Miso<SPI> {}
pub unsafe trait Mosi<SPI> {}

unsafe impl Sck<SPI1> for PA5<Alternate<AF5>> {}
// unsafe impl Sck<SPI1> for PB3<Alternate<AF5>> {}

unsafe impl Sck<SPI2> for PB13<Alternate<AF5>> {}

// unsafe impl Sck<SPI3> for PB3<Alternate<AF6>> {}
unsafe impl Sck<SPI3> for PC10<Alternate<AF6>> {}

unsafe impl Miso<SPI1> for PA6<Alternate<AF5>> {}
// unsafe impl Miso<SPI1> for PB4<Alternate<AF5>> {}

unsafe impl Miso<SPI2> for PB14<Alternate<AF5>> {}

// unsafe impl Miso<SPI3> for PB4<Alternate<AF6>> {}
unsafe impl Miso<SPI3> for PC11<Alternate<AF6>> {}

unsafe impl Mosi<SPI1> for PA7<Alternate<AF5>> {}
unsafe impl Mosi<SPI1> for PB5<Alternate<AF5>> {}

unsafe impl Mosi<SPI2> for PB15<Alternate<AF5>> {}

unsafe impl Mosi<SPI3> for PB5<Alternate<AF6>> {}
unsafe impl Mosi<SPI3> for PC12<Alternate<AF6>> {}

pub struct Spi<SPI, PINS> {
    spi: SPI,
    pins: PINS,
}

macro_rules! hal {
    ($($SPIX:ident: ($spiX:ident, $APBX:ident, $spiXen:ident, $spiXrst:ident, $pclkX:ident),)+) => {
        $(
            impl<SCK, MISO, MOSI> Spi<$SPIX, (SCK, MISO, MOSI)> {
                pub fn $spiX<F>(
                    spi: $SPIX,
                    pins: (SCK, MISO, MOSI),
                    mode: Mode,
                    freq: F,
                    clocks: Clocks,
                    apb2: &mut $APBX,
                ) -> Self
                where
                    F: Into<Hertz>,
                    SCK: Sck<$SPIX>,
                    MISO: Miso<$SPIX>,
                    MOSI: Mosi<$SPIX>,
                {
                    // enable or reset $SPIX
                    apb2.enr().modify(|_, w| w.$spiXen().enabled());
                    apb2.rstr().modify(|_, w| w.$spiXrst().set_bit());
                    apb2.rstr().modify(|_, w| w.$spiXrst().clear_bit());

                    // FRXTH: RXNE event is generated if the FIFO level is greater than or equal to 8-bit
                    // DS: 8-bit data size
                    // SSOE: Slave Select output disabled
                    spi.cr2
                        .write(|w| unsafe { w.frxth().set_bit().ds().bits(0b111).ssoe().clear_bit() });

                    let br = match clocks.$pclkX().0 / freq.into().0 {
                        0 => unreachable!(),
                        1...2 => 0b000,
                        3...5 => 0b001,
                        6...11 => 0b010,
                        12...23 => 0b011,
                        24...39 => 0b100,
                        40...95 => 0b101,
                        96...191 => 0b110,
                        _ => 0b111,
                    };

                    // CPHA: phase
                    // CPOL: polarity
                    // MSTR: master mode
                    // BR: 1 MHz
                    // SPE: SPI disabled
                    // LSBFIRST: MSB first
                    // SSM: enable software slave management (NSS pin free for other uses)
                    // SSI: set nss high = master mode
                    // CRCEN: hardware CRC calculation disabled
                    // BIDIMODE: 2 line unidirectional (full duplex)
                    spi.cr1.write(|w| unsafe {
                        w.cpha()
                            .bit(mode.phase == Phase::CaptureOnSecondTransition)
                            .cpol()
                            .bit(mode.polarity == Polarity::IdleHigh)
                            .mstr()
                            .set_bit()
                            .br()
                            .bits(br)
                            .spe()
                            .set_bit()
                            .lsbfirst()
                            .clear_bit()
                            .ssi()
                            .set_bit()
                            .ssm()
                            .set_bit()
                            .crcen()
                            .clear_bit()
                            .bidimode()
                            .clear_bit()
                    });

                    Spi { spi, pins }
                }

                pub fn free(self) -> ($SPIX, (SCK, MISO, MOSI)) {
                    (self.spi, self.pins)
                }
            }

            impl<PINS> FullDuplex<u8> for Spi<$SPIX, PINS> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    let sr = self.spi.sr.read();

                    Err(if sr.ovr().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if sr.modf().bit_is_set() {
                        nb::Error::Other(Error::ModeFault)
                    } else if sr.crcerr().bit_is_set() {
                        nb::Error::Other(Error::Crc)
                    } else if sr.rxne().bit_is_set() {
                        // NOTE(read_volatile) read only 1 byte (the svd2rust API only allows reading a half-word)
                        return Ok(unsafe { ptr::read_volatile(&self.spi.dr as *const _ as *const u8) });
                    } else {
                        nb::Error::WouldBlock
                    })
                }

                fn send(&mut self, byte: u8) -> nb::Result<(), Error> {
                    let sr = self.spi.sr.read();

                    Err(if sr.ovr().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if sr.modf().bit_is_set() {
                        nb::Error::Other(Error::ModeFault)
                    } else if sr.crcerr().bit_is_set() {
                        nb::Error::Other(Error::Crc)
                    } else if sr.txe().bit_is_set() {
                        // NOTE(write_volatile) see note above
                        unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, byte) }
                        return Ok(());
                    } else {
                        nb::Error::WouldBlock
                    })
                }
            }

            impl<PINS> ::hal::blocking::spi::transfer::Default<u8> for Spi<$SPIX, PINS> {}

            impl<PINS> ::hal::blocking::spi::write::Default<u8> for Spi<$SPIX, PINS> {}
        )+
    }
}

hal! {
    SPI1: (spi1, APB2, spi1en, spi1rst, pclk2),
    SPI2: (spi2, APB1, spi2en, spi2rst, pclk1),
    SPI3: (spi3, APB1, spi3en, spi3rst, pclk1),
}

// FIXME not working
// impl ::hal::blocking::spi::Write<u8> for Spi {
//     type Error = Error;

//     fn write(&mut self, bytes: &[u8]) -> Result<(), Error> {
//         for byte in bytes {
//             'l: loop {
//                 let sr = self.spi.sr.read();

//                 // ignore overruns because we don't care about the incoming data
//                 // if sr.ovr().bit_is_set() {
//                 // Err(nb::Error::Other(Error::Overrun))
//                 // } else
//                 if sr.modf().bit_is_set() {
//                     return Err(Error::ModeFault);
//                 } else if sr.crcerr().bit_is_set() {
//                     return Err(Error::Crc);
//                 } else if sr.txe().bit_is_set() {
//                     // NOTE(write_volatile) see note above
//                     unsafe { ptr::write_volatile(&self.spi.dr as *const _ as *mut u8, *byte) }
//                     break 'l;
//                 } else {
//                     // try again
//                 }
//             }
//         }

//         // wait until the transmission of the last byte is done
//         while self.spi.sr.read().bsy().bit_is_set() {}

//         // clear OVR flag
//         unsafe {
//             ptr::read_volatile(&self.spi.dr as *const _ as *const u8);
//         }
//         self.spi.sr.read();

//         Ok(())
//     }
// }
