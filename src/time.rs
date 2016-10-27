//! Temporal quantification

use peripheral;
use peripheral::gptim::CntR;

/// The frequency, in Hz, of the clock that backs up this module.
///
/// The inverse of this value is the length of a single tick in seconds.
pub const FREQUENCY: u32 = ::APB1_CLOCK;

/// A measurement of a monotonically increasing clock
#[derive(Clone, Copy)]
pub struct Instant {
    cnt: CntR,
}

impl Instant {
    /// Returns the instant corresponding to "now".
    pub fn now() -> Self {
        Instant { cnt: peripheral::tim2().cnt.read() }
    }

    /// Returns the number of "ticks" that have passed since this instant was
    /// created.
    ///
    /// # Caveats
    ///
    /// This function doesn't handle overflows. There's no way to know if more
    /// than `1 << 32` ticks have passed; the number of ticks will just wrap
    /// around.
    pub fn elapsed(&self) -> u32 {
        let cnt = peripheral::tim2().cnt.read();

        let before = (u32::from(self.cnt.cnth()) << 16) +
                     u32::from(self.cnt.cntl());
        let now = (u32::from(cnt.cnth()) << 16) + u32::from(cnt.cntl());

        now.wrapping_sub(before)
    }
}

/// Initializes the necessary stuff to be able to use the `time` module
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let rcc = peripheral::rcc_mut();
    let tim2 = peripheral::tim2_mut();

    rcc.apb1enr.modify(|_, w| w.tim2en(true));
    tim2.cr1.write(|w| w.opm(false).cen(true));
}
