use core::ptr;
use core::marker::PhantomData;

use hal::serial;
use nb;
use stm32f30x::{USART1, USART2, USART3};

use gpio::GPIOA::{PA10, PA2, PA3, PA9};
use gpio::GPIOB::{PB10, PB11, PB6, PB7};
use gpio::GPIOC::{PC10, PC11, PC4, PC5};
use gpio::GPIOD::{PD5, PD6, PD8, PD9};
use gpio::GPIOE::{PE0, PE1, PE15};
use gpio::{AF7, Alternate};
use rcc::{APB1, APB2, Clocks};
use time::Bps;

#[derive(Debug)]
pub enum Error {
    Framing,
    Noise,
    Overrun,
    Parity,
    #[doc(hidden)] _Extensible,
}

/// Implementation detail - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<USART> {}

/// Implementation detail - DO NOT IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<USART> {}

unsafe impl TxPin<USART1> for PA9<Alternate<AF7>> {}
unsafe impl TxPin<USART1> for PB6<Alternate<AF7>> {}
unsafe impl TxPin<USART1> for PC4<Alternate<AF7>> {}
unsafe impl TxPin<USART1> for PE0<Alternate<AF7>> {}

unsafe impl RxPin<USART1> for PA10<Alternate<AF7>> {}
unsafe impl RxPin<USART1> for PB7<Alternate<AF7>> {}
unsafe impl RxPin<USART1> for PC5<Alternate<AF7>> {}
unsafe impl RxPin<USART1> for PE1<Alternate<AF7>> {}

unsafe impl TxPin<USART2> for PA2<Alternate<AF7>> {}
// unsafe impl TxPin<USART2> for PA14<Alternate<AF7>> {}
// unsafe impl TxPin<USART2> for PB3<Alternate<AF7>> {}
unsafe impl TxPin<USART2> for PD5<Alternate<AF7>> {}

unsafe impl RxPin<USART2> for PA3<Alternate<AF7>> {}
// unsafe impl RxPin<USART2> for PA15<Alternate<AF7>> {}
// unsafe impl RxPin<USART2> for PB4<Alternate<AF7>> {}
unsafe impl RxPin<USART2> for PD6<Alternate<AF7>> {}

unsafe impl TxPin<USART3> for PB10<Alternate<AF7>> {}
unsafe impl TxPin<USART3> for PC10<Alternate<AF7>> {}
unsafe impl TxPin<USART3> for PD8<Alternate<AF7>> {}

unsafe impl RxPin<USART3> for PB11<Alternate<AF7>> {}
unsafe impl RxPin<USART3> for PC11<Alternate<AF7>> {}
unsafe impl RxPin<USART3> for PD9<Alternate<AF7>> {}
unsafe impl RxPin<USART3> for PE15<Alternate<AF7>> {}

pub struct Serial<USART, PINS> {
    usart: USART,
    pins: PINS,
}

pub struct Rx<USART> {
    _usart: PhantomData<USART>,
}

pub struct Tx<USART> {
    _usart: PhantomData<USART>,
}

macro_rules! hal {
    ($(
        $USARTX:ident: ($usartX:ident, $APB:ident, $usartXen:ident, $usartXrst:ident, $pclkX:ident),
    )+) => {
        $(
            impl<TX, RX> Serial<$USARTX, (TX, RX)> {
                pub fn $usartX(
                    usart: $USARTX,
                    baud_rate: Bps,
                    pins: (TX, RX),
                    apb: &mut $APB,
                    clocks: Clocks,
                ) -> Self
                where
                    TX: TxPin<$USARTX>,
                    RX: RxPin<$USARTX>,
                {
                    // enable or reset $USARTX
                    apb.enr().modify(|_, w| w.$usartXen().enabled());
                    apb.rstr().modify(|_, w| w.$usartXrst().set_bit());
                    apb.rstr().modify(|_, w| w.$usartXrst().clear_bit());

                    // disable hardware flow control
                    // TODO enable DMA
                    // usart.cr3.write(|w| w.rtse().clear_bit().ctse().clear_bit());

                    let brr = clocks.$pclkX().0 / baud_rate.0;
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

                pub fn split(self) -> (Tx<$USARTX>, Rx<$USARTX>) {
                    (
                        Tx {
                            _usart: PhantomData,
                        },
                        Rx {
                            _usart: PhantomData,
                        },
                    )
                }

                pub fn free(self) -> ($USARTX, (TX, RX)) {
                    (self.usart, self.pins)
                }
            }

            impl serial::Read<u8> for Rx<$USARTX> {
                type Error = Error;

                fn read(&mut self) -> nb::Result<u8, Error> {
                    // NOTE(unsafe) atomic read with no side effects
                    let isr = unsafe { (*$USARTX::ptr()).isr.read() };

                    Err(if isr.pe().bit_is_set() {
                        nb::Error::Other(Error::Parity)
                    } else if isr.fe().bit_is_set() {
                        nb::Error::Other(Error::Framing)
                    } else if isr.nf().bit_is_set() {
                        nb::Error::Other(Error::Noise)
                    } else if isr.ore().bit_is_set() {
                        nb::Error::Other(Error::Overrun)
                    } else if isr.rxne().bit_is_set() {
                        // NOTE(read_volatile) see `write_volatile` below
                        return Ok(unsafe {
                            ptr::read_volatile(&(*$USARTX::ptr()).rdr as *const _ as *const _)
                        });
                    } else {
                        nb::Error::WouldBlock
                    })
                }
            }

            impl serial::Write<u8> for Tx<$USARTX> {
                // NOTE(!) See section "29.7 USART interrupts"; the only possible errors during transmission
                // are: clear to send (which is disabled in this case) errors and framing errors (which only
                // occur in SmartCard mode); neither of these apply to our hardware configuration
                type Error = !;

                fn flush(&mut self) -> nb::Result<(), !> {
                    // NOTE(unsafe) atomic read with no side effects
                    let isr = unsafe { (*$USARTX::ptr()).isr.read() };

                    if isr.tc().bit_is_set() {
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }

                fn write(&mut self, byte: u8) -> nb::Result<(), !> {
                    // NOTE(unsafe) atomic read with no side effects
                    let isr = unsafe { (*$USARTX::ptr()).isr.read() };

                    if isr.txe().bit_is_set() {
                        // NOTE(unsafe) atomic write to stateless register
                        // NOTE(write_volatile) 8-bit write that's not possible through the svd2rust API
                        unsafe {
                            ptr::write_volatile(&(*$USARTX::ptr()).tdr as *const _ as *mut _, byte)
                        }
                        Ok(())
                    } else {
                        Err(nb::Error::WouldBlock)
                    }
                }
            }
        )+
    }
}

hal! {
    USART1: (usart1, APB2, usart1en, usart1rst, pclk2),
    USART2: (usart2, APB1, usart2en, usart2rst, pclk1),
    USART3: (usart3, APB1, usart3en, usart3rst, pclk1),
}
