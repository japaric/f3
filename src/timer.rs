//! Periodic timer

use core::u16;

use cast::{u16, u32};
use stm32f30x::{RCC, TIM7};

use frequency;

/// Specialized `Result` type
pub type Result<T> = ::core::result::Result<T, Error>;

/// An error
pub struct Error {
    _0: (),
}

/// Periodic timer
///
/// # Interrupts
///
/// - `Tim7` - update event
pub struct Timer<'a>(pub &'a TIM7);

impl<'a> Timer<'a> {
    /// Initializes the timer with a periodic timeout of `frequency` Hz
    ///
    /// NOTE After initialization, the timer will be in the paused state.
    pub fn init(&self, rcc: &RCC, frequency: u32) {
        let tim7 = self.0;

        // Power up peripherals
        rcc.apb1enr.modify(|_, w| w.tim7en().enabled());

        let ratio = frequency::APB1 / frequency;
        let psc = u16((ratio - 1) / u32(u16::MAX)).unwrap();
        tim7.psc.write(|w| w.psc().bits(psc));
        let arr = u16(ratio / u32(psc + 1)).unwrap();
        tim7.arr.write(|w| w.arr().bits(arr));

        tim7.dier.write(|w| w.uie().set_bit());
        tim7.cr1.write(|w| w.opm().continuous());
    }

    /// Clears the update event flag
    ///
    /// Returns `Err` if no update event has occurred
    pub fn clear_update_flag(&self) -> Result<()> {
        let tim7 = self.0;

        if tim7.sr.read().uif().is_no_update() {
            Err(Error { _0: () })
        } else {
            self.0.sr.modify(|_, w| w.uif().clear());
            Ok(())
        }
    }

    /// Resumes the timer count
    pub fn resume(&self) {
        self.0.cr1.modify(|_, w| w.cen().enabled());
    }

    /// Pauses the timer
    pub fn pause(&self) {
        self.0.cr1.modify(|_, w| w.cen().disabled());
    }
}
