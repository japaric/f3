//! Floating Point Unit (FPU)

use cortex_m;

/// Initializes the FPU
///
/// # Safety
///
/// - Must be called once
/// - Must be called in an interrupt-free environment
pub unsafe fn init() {
    let scb = cortex_m::peripheral::scb_mut();

    // Enable the FPU co-processor
    let cpacr = scb.cpacr.read();
    scb.cpacr.write({
        // Enable privileged access to the co-processor 10
        const CP10: u32 = 0b01 << 20;
        // Enable privileged access to the co-processor 11
        const CP11: u32 = 0b01 << 22;

        cpacr | CP10 | CP11
    });
}
