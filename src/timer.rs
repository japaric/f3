//! Timers

use core::marker::PhantomData;
use core::mem;

use cortex_m::interrupt::Mutex;
use futuro::prelude::{Async, Future, InfiniteStream};

use peripheral;

/// A timer
pub struct Timer {
    _0: (),
}

enum Mode {
    OneShot,
    Periodic,
}

fn configure(ms: u16, mode: Mode) {
    let tim7 = unsafe { peripheral::tim7_mut() };

    // Set timeout to `ms` milliseconds
    tim7.arr.write(|w| w.arr(ms));
    tim7.egr.write(|w| w.ug(true));

    // Clear any previous update event by clearing the update event flag
    // (UIF)
    tim7.sr.read();
    tim7.sr.write(|w| w);

    // OPM: One pulse mode (`true`) or Continuous mode (`true`)
    // CEN: enable the counter
    tim7.cr1.write(|w| {
        w.cen(true).opm(match mode {
            Mode::OneShot => true,
            Mode::Periodic => false,

        })
    });
}

impl Timer {
    /// Initializes the Timer peripheral
    ///
    /// NOTE There can only be an instance of this peripheral. If you call this
    /// constructor a second time, it will return `None`.
    pub fn new() -> Option<Self> {
        static YIELDED: Mutex<bool> = Mutex::new(false);

        YIELDED.lock(|yielded| {
            unsafe {
                if *yielded {
                    None
                } else {
                    *yielded = true;

                    let rcc = peripheral::rcc_mut();
                    let tim7 = peripheral::tim7_mut();

                    // Enable TIM7
                    rcc.apb1enr.modify(|_, w| w.tim7en(true));

                    // Set pre-scaler to 8_000 -> Frequency = 1 KHz
                    tim7.psc.write(|w| w.psc(7_999));

                    Some(Timer { _0: () })
                }
            }
        })
    }

    /// Sets a one time "alarm"
    pub fn oneshot(self, ms: u16) -> OneShot {
        configure(ms, Mode::OneShot);

        OneShot { timer: Some(self) }
    }

    /// Sets a periodic "alarm"
    pub fn periodic(&mut self, ms: u16) -> Periodic {
        configure(ms, Mode::Periodic);

        Periodic { _timer: PhantomData }
    }
}

/// Future returned by [Timer::oneshot](struct.Timer.html#method.oneshot)
#[must_use = "futures do nothing unless polled"]
pub struct OneShot {
    timer: Option<Timer>,
}

impl Future for OneShot {
    type Item = Timer;

    fn poll(&mut self) -> Async<Timer> {
        let timer = mem::replace(&mut self.timer, None)
            .expect("`oneshot` cannot be polled twice");
        let tim7 = unsafe { peripheral::tim7_mut() };

        if tim7.sr.read().uif() {
            tim7.sr.write(|w| w);
            Async::Ready(timer)
        } else {
            self.timer = Some(timer);
            Async::NotReady
        }
    }
}

/// Stream returned by [Timer::periodic](struct.Timer.html#method.periodic)
#[must_use = "streams do nothing unless polled"]
pub struct Periodic<'t> {
    _timer: PhantomData<&'t mut Timer>,
}

impl<'t> InfiniteStream for Periodic<'t> {
    type Item = ();

    fn poll(&mut self) -> Async<()> {
        let tim7 = unsafe { peripheral::tim7_mut() };

        if tim7.sr.read().uif() {
            tim7.sr.write(|w| w);
            Async::Ready(())
        } else {
            Async::NotReady
        }
    }
}
