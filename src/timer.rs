use cast::{u16, u32};
use hal::{self, Timer as _Timer};
use nb;
use stm32f30x::TIM6;

use rcc::{APB1, Clocks};
use time::Hertz;

pub struct Timer {
    clocks: Clocks,
    tim: TIM6,
    timeout: Hertz,
}

impl Timer {
    pub fn new<T>(tim: TIM6, timeout: T, clocks: Clocks, apb1: &mut APB1) -> Self
    where
        T: Into<Hertz>,
    {
        if apb1.enr().read().tim6en().bit_is_set() {
            apb1.rstr().modify(|_, w| w.tim6rst().set_bit());
            apb1.rstr().modify(|_, w| w.tim6rst().clear_bit());
        } else {
            apb1.enr().modify(|_, w| w.tim6en().enabled());
        }

        // Continuous mode
        tim.cr1.write(|w| w.opm().continuous());

        // Enable update event interrupt
        tim.dier.write(|w| w.uie().set_bit());

        let mut timer = Timer {
            clocks,
            tim,
            timeout: Hertz(0),
        };
        timer.set_timeout(timeout);
        timer
    }

    pub fn free(mut self) -> TIM6 {
        self.pause();
        self.tim
    }
}

impl hal::Timer for Timer {
    type Time = Hertz;

    fn get_timeout(&self) -> Hertz {
        self.timeout
    }

    fn pause(&mut self) {
        self.tim.cr1.modify(|_, w| w.cen().disabled());
    }

    fn restart(&mut self) {
        self.tim.cnt.write(|w| unsafe { w.cnt().bits(0) });
    }

    fn resume(&mut self) {
        self.tim.cr1.modify(|_, w| w.cen().enabled());
    }

    fn set_timeout<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        self.timeout = timeout.into();

        let frequency = self.timeout.0;
        let ticks =
            self.clocks.pclk1().0 * if self.clocks.ppre1() == 1 { 1 } else { 2 } / frequency;

        let psc = u16((ticks - 1) / (1 << 16)).unwrap();
        self.tim.psc.write(|w| w.psc().bits(psc));

        let arr = u16(ticks / u32(psc + 1)).unwrap();
        self.tim.arr.write(|w| w.arr().bits(arr));
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
