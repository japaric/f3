//! I2C
//!
//! - SCL = PB6
//! - SDA = PB7

use peripheral;

use futuro::{Async, Future};
use cortex_m::interrupt::Mutex;

use core::marker::PhantomData;
use core::mem;

/// Busy bus
pub enum Busy {}

/// Free bus
pub enum Free {}

/// Slave address
pub enum Address {
    /// 7-bit address
    U7(u8),
    /// 10-bit address
    U10(u16),
}

impl Address {
    /// Creates a 10-bit address
    pub fn u10(address: u16) -> Address {
        Address::U10(address & 0b1111111111)
    }

    /// Creates a 7-bit address
    pub fn u7(address: u8) -> Address {
        Address::U7(address & 0b1111111)
    }
}

enum Mode {
    Read,
    Write,
}

fn start(address: Option<Address>, mode: Mode, n: u8) {
    let i2c1 = unsafe { peripheral::i2c1_mut() };

    match address {
        None => {
            i2c1.cr2.modify(|_, w| {
                w.nbytes(n)
                    .rd_wrn(match mode {
                        Mode::Read => true,
                        Mode::Write => false,
                    })
                    .start(true)
            })
        }
        Some(address) => {
            i2c1.cr2.write(|w| {
                w.nbytes(n);
                w.start(true);

                match mode {
                    Mode::Read => {
                        w.rd_wrn(true);
                    }
                    Mode::Write => {
                        w.rd_wrn(false);
                    }
                }

                match address {
                    Address::U10(address) => {
                        w.add10(true)
                            .sadd8((address >> 8) as u8)
                            .sadd1((address >> 1) as u8)
                            .sadd0((address & 1) == 1)
                    }
                    Address::U7(address) => w.add10(false).sadd1(address),
                }
            })
        }
    };
}

/// I2C
pub struct I2c<S> {
    _state: PhantomData<S>,
}

impl I2c<Free> {
    /// Initializes the I2C peripheral
    ///
    /// NOTE There can only be an instance of this peripheral. If you call this
    /// constructor a second time, it will return `None`.
    pub fn new() -> Option<I2c<Free>> {
        static YIELDED: Mutex<bool> = Mutex::new(false);

        YIELDED.lock(|yielded| {
            if *yielded {
                None
            } else {
                *yielded = true;

                let gpiob = unsafe { peripheral::gpiob_mut() };
                let i2c1 = unsafe { peripheral::i2c1_mut() };
                let rcc = unsafe { peripheral::rcc_mut() };

                // RCC: enable the I2C1 peripheral
                rcc.ahbenr.modify(|_, w| w.iopben(true));
                rcc.apb1enr.modify(|_, w| w.i2c1en(true));

                // GPIOB: configure PB6 and PB7 for I2C use
                // AFRL6 = 4 (I2C1_SCL)
                // AFRL7 = 4 (I2C1_SDA)
                // MODER* = 0b10 (Alternate function)
                gpiob.afrl.modify(|_, w| w.afrl6(4).afrl7(4));
                gpiob.moder.modify(|_, w| w.moder6(0b10).moder7(0b10));

                // Configure for "fast mode" (400 KHz)
                // PRESC:  t_I2CCLK = (0 + 1) / 8 MHz = 125 ns
                // SCLL:   t_SCLL   = (9 + 1) * t_I2CCLK = 1.25 us
                // SCLH:   t_SCLH   = (3 + 1) * t_I2CCLK = 0.5 us
                //
                // t_SYNC1 + t_SYNC2 > 4 * t_I2CCLK = 0.5 us
                // t_SCL = t_SYNC1 + t_SYNC2 t_SCLL + t_SCLH ~= 2.5 us
                i2c1.timingr
                    .write(|w| w.presc(0).scll(9).sclh(3).sdadel(1).scldel(3));
                i2c1.cr2.write(|w| w.add10(false));

                // Enable the peripheral
                i2c1.cr1.write(|w| w.pe(true));

                Some(I2c { _state: PhantomData })
            }
        })
    }

    /// Reads a single byte from a slave with a certain `address`
    pub fn read(self, address: Address) -> Read {
        start(Some(address), Mode::Read, 1);

        Read { i2c: Some(I2c { _state: PhantomData }) }
    }

    /// Reads bytes from a slave with a certain `address` into a `buffer`
    pub fn read_exact<B>(self, address: Address, mut buffer: B) -> ReadExact<B>
        where B: AsMut<[u8]>
    {
        start(Some(address), Mode::Read, buffer.as_mut().len() as u8);

        ReadExact {
            pos: 0,
            state: Some((I2c { _state: PhantomData }, buffer)),
        }
    }

    /// Sends a single `byte` to a slave with a certain `address`
    pub fn write(self, address: Address, byte: u8) -> Write {
        start(Some(address), Mode::Write, 1);

        Write {
            byte: byte,
            i2c: Some(I2c { _state: PhantomData }),
        }
    }

    /// Sends several `bytes` to a slave with a certain `address`
    // XXX doesn't work if `bytes.len() > 255`
    pub fn write_all<B>(self, address: Address, bytes: B) -> WriteAll<B>
        where B: AsRef<[u8]>
    {
        start(Some(address), Mode::Write, bytes.as_ref().len() as u8);

        WriteAll {
            pos: 0,
            state: Some((I2c { _state: PhantomData }, bytes)),
        }
    }
}

/// Future returned by [I2c::read](struct.I2c.html#method.read)
#[must_use = "futures do nothing unless polled"]
pub struct Read {
    i2c: Option<I2c<Busy>>,
}

impl Future for Read {
    type Item = (I2c<Busy>, u8);

    fn poll(&mut self) -> Async<Self::Item> {
        let i2c = mem::replace(&mut self.i2c, None)
            .expect("`read` can't be polled twice");

        let i2c1 = unsafe { peripheral::i2c1_mut() };
        if i2c1.isr.read().rxne() {
            Async::Ready((i2c, i2c1.rxdr.read().rxdata()))
        } else {
            self.i2c = Some(i2c);
            Async::NotReady
        }
    }
}

/// Future returned by [I2c::read_exact](struct.I2c.html#method.read_exact)
#[must_use = "futures do nothing unless polled"]
pub struct ReadExact<B> {
    pos: u8,
    state: Option<(I2c<Busy>, B)>,
}

impl<B> Future for ReadExact<B>
    where B: AsMut<[u8]>
{
    type Item = (I2c<Busy>, B);

    fn poll(&mut self) -> Async<Self::Item> {
        let (i2c, mut buffer) = mem::replace(&mut self.state, None)
            .expect("`read_exact` can't be polled twice");

        let i2c1 = unsafe { peripheral::i2c1_mut() };
        let mut ready = false;
        if i2c1.isr.read().rxne() {
            if let Some(slot) = buffer.as_mut().get_mut(usize::from(self.pos)) {
                *slot = i2c1.rxdr.read().rxdata();
                self.pos += 1;
            } else {
                ready = true;
            }
        }

        let n = buffer.as_mut().len() as u8;
        if self.pos == n || ready {
            Async::Ready((i2c, buffer))
        } else {
            self.state = Some((i2c, buffer));
            Async::NotReady
        }
    }
}

/// Future returned by [I2c::stop](struct.I2c.html#method.stop)
#[must_use = "futures do nothing unless polled"]
pub struct Stop {
    i2c: Option<I2c<Free>>,
}

impl Future for Stop {
    type Item = I2c<Free>;

    fn poll(&mut self) -> Async<Self::Item> {
        let i2c = mem::replace(&mut self.i2c, None)
            .expect("`stop` can't be polled twice");

        let i2c1 = unsafe { peripheral::i2c1_mut() };
        if i2c1.isr.read().tc() {
            i2c1.cr2.modify(|_, w| w.stop(true));
            Async::Ready(i2c)
        } else {
            self.i2c = Some(i2c);
            Async::NotReady
        }
    }
}

/// Future returned by [I2c::write](struct.I2c.html#method.write)
#[must_use = "futures do nothing unless polled"]
pub struct Write {
    byte: u8,
    i2c: Option<I2c<Busy>>,
}

impl Future for Write {
    type Item = I2c<Busy>;

    fn poll(&mut self) -> Async<Self::Item> {
        let i2c = mem::replace(&mut self.i2c, None)
            .expect("`write` can't be polled twice");

        let i2c1 = unsafe { peripheral::i2c1_mut() };
        if i2c1.isr.read().txis() {
            i2c1.txdr.write(|w| w.txdata(self.byte));
            Async::Ready(i2c)
        } else {
            self.i2c = Some(i2c);
            Async::NotReady
        }
    }
}

/// Future returned by [I2c::write_all](struct.I2c.html#method.write_all)
#[must_use = "futures do nothing unless polled"]
pub struct WriteAll<B> {
    state: Option<(I2c<Busy>, B)>,
    pos: u8,
}

impl<B> Future for WriteAll<B>
    where B: AsRef<[u8]>
{
    type Item = I2c<Busy>;

    fn poll(&mut self) -> Async<Self::Item> {
        let (i2c, bytes) = mem::replace(&mut self.state, None)
            .expect("`write_all` can't be polled twice");

        let i2c1 = unsafe { peripheral::i2c1_mut() };
        if i2c1.isr.read().txis() {
            if let Some(byte) = bytes.as_ref().get(usize::from(self.pos)) {
                i2c1.txdr.write(|w| w.txdata(*byte));
                self.pos += 1;
            } else {
                return Async::Ready(i2c);
            }
        }

        if self.pos == bytes.as_ref().len() as u8 {
            Async::Ready(i2c)
        } else {
            self.state = Some((i2c, bytes));
            Async::NotReady
        }
    }
}

impl I2c<Busy> {
    /// Waits until the current transmission ends
    pub fn flush(self) -> Flush {
        Flush { i2c: Some(self) }
    }

    /// Reads a single byte from a slave into `buffer`
    ///
    /// - `address = None`, to continue communicating with the same slave
    /// - `address = Some(_)`, to switch to another slave without releasing the
    ///   bus
    pub fn read(self, address: Option<Address>) -> Read {
        start(address, Mode::Read, 1);

        Read { i2c: Some(I2c { _state: PhantomData }) }

    }

    /// Reads several bytes from a slave into `buffer`
    ///
    /// - `address = None`, to continue communicating with the same slave
    /// - `address = Some(_)`, to switch to another slave without releasing the
    ///   bus
    pub fn read_exact<B>(self,
                         address: Option<Address>,
                         mut buffer: B)
                         -> ReadExact<B>
        where B: AsMut<[u8]>
    {
        start(address, Mode::Read, buffer.as_mut().len() as u8);

        ReadExact {
            pos: 0,
            state: Some((I2c { _state: PhantomData }, buffer)),
        }
    }

    /// Sends a single byte to a slave
    ///
    /// - `address = None`, to continue communicating with the same slave
    /// - `address = Some(_)`, to switch to another slave without releasing the
    ///   bus
    pub fn write(self, address: Option<Address>, byte: u8) -> Write {
        start(address, Mode::Write, 1);

        Write {
            byte: byte,
            i2c: Some(I2c { _state: PhantomData }),
        }

    }

    /// Sends `bytes` to a slave
    ///
    /// - `address = None`, to continue communicating with the same slave
    /// - `address = Some(_)`, to switch to another slave without releasing the
    ///   bus
    // XXX doesn't work if `bytes.len() > 255`
    pub fn write_all<B>(self, address: Option<Address>, bytes: B) -> WriteAll<B>
        where B: AsRef<[u8]>
    {
        start(address, Mode::Write, bytes.as_ref().len() as u8);

        WriteAll {
            pos: 0,
            state: Some((I2c { _state: PhantomData }, bytes)),
        }
    }

    /// Ends the I2C transmission and frees the bus
    pub fn stop(self) -> Stop {
        Stop { i2c: Some(I2c { _state: PhantomData }) }
    }
}

/// Future returned by [I2c::flush](struct.I2c.html#method.flush)
#[must_use = "futures do nothing unless polled"]
pub struct Flush {
    i2c: Option<I2c<Busy>>,
}

impl Future for Flush {
    type Item = I2c<Busy>;

    fn poll(&mut self) -> Async<Self::Item> {
        let i2c = mem::replace(&mut self.i2c, None)
            .expect("`flush` can't be polled twice");

        let i2c1 = unsafe { peripheral::i2c1_mut() };
        if i2c1.isr.read().tc() {
            Async::Ready(i2c)
        } else {
            self.i2c = Some(i2c);
            Async::NotReady
        }
    }
}
