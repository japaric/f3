use stm32f30x::I2C1;

use gpio::GPIOB::{PB6, PB7};
use gpio::{AF4, Alternate};
use hal::blocking::i2c::{Write, WriteRead};
use rcc::APB1;

#[derive(Debug)]
pub enum Error {
    /// Bus error
    Bus,
    /// Arbitration loss
    Arbitration,
    // Overrun, // slave mode only
    // Pec, // SMBUS mode only
    // Timeout, // SMBUS mode only
    // Alert, // SMBUS mode only
    #[doc(hidden)] _Extensible,
}

pub struct I2c {
    i2c: I2C1,
}

impl I2c {
    pub fn new(
        i2c: I2C1,
        (_scl, _sda): (PB6<Alternate<AF4>>, PB7<Alternate<AF4>>),
        apb1: &mut APB1,
    ) -> Self {
        if apb1.enr().read().i2c1en().bit_is_set() {
            apb1.rstr().modify(|_, w| w.i2c1rst().set_bit());
            apb1.rstr().modify(|_, w| w.i2c1rst().clear_bit());
        } else {
            apb1.enr().modify(|_, w| w.i2c1en().enabled());
        }

        // Configure for "fast mode" (400 KHz)
        // PRESC:  t_I2CCLK = (0 + 1) / 8 MHz = 125 ns
        // SCLL:   t_SCLL   = (9 + 1) * t_I2CCLK = 1.25 us
        // SCLH:   t_SCLH   = (3 + 1) * t_I2CCLK = 0.5 us
        //
        // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK = 0.5 us
        // t_SCL = t_SYNC1 + t_SYNC2 t_SCLL + t_SCLH ~= 2.5 us
        i2c.timingr.write(|w| unsafe {
            w.presc()
                .bits(0)
                .scll()
                .bits(9)
                .sclh()
                .bits(3)
                .sdadel()
                .bits(1)
                .scldel()
                .bits(3)
        });

        // Enable the peripheral
        i2c.cr1.write(|w| w.pe().set_bit());

        I2c { i2c }
    }
}

macro_rules! busy_wait {
    ($i2c:expr, $flag:ident) => {
        loop {
            let isr = $i2c.isr.read();

            if isr.berr().bit_is_set() {
                return Err(Error::Bus);
            } else if isr.arlo().bit_is_set() {
                return Err(Error::Arbitration);
            } else if isr.$flag().bit_is_set() {
                break;
            } else {
                // try again
            }
        }
    }
}

impl Write for I2c {
    type Error = Error;

    fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
        // TODO support transfers of more than 255 bytes
        assert!(bytes.len() < 256 && bytes.len() > 0);

        // START and prepare to send `bytes`
        self.i2c.cr2.write(|w| unsafe {
            w.sadd1()
                .bits(addr)
                .rd_wrn()
                .clear_bit()
                .nbytes()
                .bits(bytes.len() as u8)
                .start()
                .set_bit()
                .autoend()
                .set_bit()
        });

        for byte in bytes {
            // Wait until we are allowed to send data (START has been ACKed or last byte when through)
            busy_wait!(self.i2c, txis);

            // put byte on the wire
            self.i2c.txdr.write(|w| unsafe { w.txdata().bits(*byte) });
        }

        // Wait until the last transmission is finished ???
        // busy_wait!(self.i2c, busy);

        // automatic STOP

        Ok(())
    }
}

impl WriteRead for I2c {
    type Error = Error;

    fn write_read(&mut self, addr: u8, bytes: &[u8], buffer: &mut [u8]) -> Result<(), Error> {
        // TODO support transfers of more than 255 bytes
        assert!(bytes.len() < 256 && bytes.len() > 0);
        assert!(buffer.len() < 256 && buffer.len() > 0);

        // TODO do we have to explicitly wait if the bus is busy (e.g. another master is
        // communicating)?

        // START and prepare to send `bytes`
        self.i2c.cr2.write(|w| unsafe {
            w.sadd1()
                .bits(addr)
                .rd_wrn()
                .clear_bit()
                .nbytes()
                .bits(bytes.len() as u8)
                .start()
                .set_bit()
                .autoend()
                .clear_bit()
        });

        for byte in bytes {
            // Wait until we are allowed to send data (START has been ACKed or last byte when through)
            busy_wait!(self.i2c, txis);

            // put byte on the wire
            self.i2c.txdr.write(|w| unsafe { w.txdata().bits(*byte) });
        }

        // Wait until the last transmission is finished
        busy_wait!(self.i2c, tc);

        // reSTART and prepare to receive bytes into `buffer`
        self.i2c.cr2.write(|w| unsafe {
            w.sadd1()
                .bits(addr)
                .rd_wrn()
                .set_bit()
                .nbytes()
                .bits(buffer.len() as u8)
                .start()
                .set_bit()
                .autoend()
                .set_bit()
        });

        for byte in buffer {
            // Wait until we have received something
            busy_wait!(self.i2c, rxne);

            *byte = self.i2c.rxdr.read().rxdata().bits();
        }

        // automatic STOP

        Ok(())
    }
}
