//! Time sections of code

use peripheral;

/// The frequency, in Hz, of the clock that backs up this module.
///
/// The inverse of this value is the length of a single tick in seconds.
pub const FREQUENCY: u32 = ::APB1_CLOCK;

/// Returns the number of "ticks" that elapsed during the execution of `f`
///
/// # Caveats
///
/// This function doesn't handle overflows. There's no way to know if more than
/// `1 << 32` ticks have passed; the number of ticks will just wrap around.
pub fn timeit<F>(f: F) -> u32
    where F: FnOnce()
{
    let tim2 = peripheral::tim2();
    let before = tim2.cnt.read();
    f();
    let after = tim2.cnt.read();

    let before = ((before.cnth() as u32) << 16) + before.cntl() as u32;
    let after = ((after.cnth() as u32) << 16) + after.cntl() as u32;

    after.wrapping_sub(before)
}

/// Initializes the necessary stuff to be able to use the `timeit` module
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
