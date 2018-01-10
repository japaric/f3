use cast::{u16, u32};
use hal::{self, Timer as _Timer};
use nb;
use stm32f30x::{TIM2, TIM3, TIM4, TIM6, TIM7};

use rcc::{APB1, Clocks};
use time::Hertz;

pub struct Timer<TIM> {
    clocks: Clocks,
    tim: TIM,
    timeout: Hertz,
}

macro_rules! hal {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident),)+) => {
        $(
            impl hal::Timer for Timer<$TIM> {
                type Time = Hertz;

                fn get_timeout(&self) -> Hertz {
                    self.timeout
                }

                fn pause(&mut self) {
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                }

                fn restart(&mut self) {
                    self.tim.cnt.reset();
                }

                fn resume(&mut self) {
                    self.tim.cr1.modify(|_, w| w.cen().set_bit());
                }

                // NOTE(allow) `w.psc().bits()` is safe for TIM{6,7} but not for TIM{2,3,4} due to
                // some SVD omission
                #[allow(unused_unsafe)]
                fn set_timeout<T>(&mut self, timeout: T)
                where
                    T: Into<Hertz>,
                {
                    self.timeout = timeout.into();

                    let frequency = self.timeout.0;
                    let ticks = self.clocks.pclk1().0 * if self.clocks.ppre1() == 1 { 1 } else { 2 }
                        / frequency;

                    let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                    self.tim.psc.write(|w| unsafe { w.psc().bits(psc) });

                    let arr = u16(ticks / u32(psc + 1)).unwrap();
                    self.tim.arr.write(|w| unsafe { w.bits(u32(arr)) });
                }

                fn wait(&mut self) -> nb::Result<(), !> {
                    if self.tim.sr.read().uif().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.sr.modify(|_, w| w.uif().clear_bit());
                        Ok(())
                    }
                }
            }

            impl Timer<$TIM> {
                // XXX(why not name this `new`?) bummer: constructors need to have different names
                // even if the `$TIM` are non overlapping (compare to the `free` function below
                // which just works)
                pub fn $tim<T>(tim: $TIM, timeout: T, clocks: Clocks, apb1: &mut APB1) -> Self
                where
                    T: Into<Hertz>,
                {
                    // enable and reset peripheral to a clean slate state
                    apb1.enr().write(|w| w.$timXen().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().set_bit());
                    apb1.rstr().modify(|_, w| w.$timXrst().clear_bit());

                    let mut timer = Timer {
                        clocks,
                        tim,
                        timeout: Hertz(0),
                    };
                    timer.set_timeout(timeout);

                    // Enable update event interrupt
                    timer.tim.dier.write(|w| w.uie().set_bit());

                    timer
                }

                pub fn free(mut self) -> $TIM {
                    self.pause();
                    self.tim
                }
            }
        )+
    }
}

hal! {
    TIM2: (tim2, tim2en, tim2rst),
    TIM3: (tim3, tim3en, tim3rst),
    TIM4: (tim4, tim4en, tim4rst),
    TIM6: (tim6, tim6en, tim6rst),
    TIM7: (tim7, tim7en, tim7rst),
}
