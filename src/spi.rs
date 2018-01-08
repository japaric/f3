use core::ptr;

use hal::spi::{FullDuplex, Mode, Phase, Polarity};
use nb;
use stm32f30x::SPI1;

use gpio::{AF5, Alternate};
use gpio::GPIOA::{PA5, PA6, PA7};
use rcc::APB2;

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

pub struct Spi {
    spi: SPI1,
}

impl Spi {
    // TODO other pin maps
    // TODO configurable frequency
    pub fn new(
        spi: SPI1,
        (_sck, _miso, _mosi): (
            PA5<Alternate<AF5>>,
            PA6<Alternate<AF5>>,
            PA7<Alternate<AF5>>,
        ),
        mode: Mode,
        apb2: &mut APB2,
    ) -> Self {
        // enable or reset SPI1
        if apb2.enr().read().spi1en().bit_is_set() {
            apb2.rstr().modify(|_, w| w.spi1rst().set_bit());
            apb2.rstr().modify(|_, w| w.spi1rst().clear_bit());
        } else {
            apb2.enr().modify(|_, w| w.spi1en().enabled());
        }

        // FRXTH: RXNE event is generated if the FIFO level is greater than or equal to 8-bit
        // DS: 8-bit data size
        // SSOE: Slave Select output disabled
        spi.cr2
            .write(|w| unsafe { w.frxth().set_bit().ds().bits(0b111).ssoe().clear_bit() });

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
                .bits(0b010)
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

        Spi { spi }
    }
}

impl FullDuplex<u8> for Spi {
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

impl ::hal::blocking::spi::transfer::Default<u8> for Spi {}

impl ::hal::blocking::spi::write::Default<u8> for Spi {}

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
