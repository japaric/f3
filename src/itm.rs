//! ITM (Instrumentation Trace Macrocell)

use core::fmt::{self, Arguments, Write};

use cortex_m;

use peripheral;

struct Port {}

impl Write for Port {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let stim = &cortex_m::peripheral::itm().stim[0];

        // TODO send in 32-bit chunks
        for byte in s.bytes() {
            while !stim.is_fifo_ready() {}
            stim.write_u8(byte);
        }

        Ok(())
    }
}

/// Initializes the ITM port
pub unsafe fn init() {
    let dbgmcu = peripheral::dbgmcu_mut();
    let dcb = cortex_m::peripheral::dcb_mut();
    let itm = cortex_m::peripheral::itm_mut();

    // DBGMCU: enable asynchronous tracing
    let cr = dbgmcu.cr.read();
    dbgmcu.cr.write({
        // Enable tracing
        const TRACE_IOEN: u32 = 1 << 5;
        // Asynchronous mode - for use with SWO
        const TRACE_MODE: u32 = 0b00 << 6;
        const TRACE_MODE_MASK: u32 = 0b11 << 6;

        ((cr | TRACE_IOEN) & !TRACE_MODE_MASK) | TRACE_MODE
    });

    // DCB: enable the ITM
    let demcr = dcb.demcr.read();
    dcb.demcr.write({
        // Enable DWT and ITM
        const TRCENA: u32 = 1 << 24;

        demcr | TRCENA
    });

    // ITM: unlock the peripheral
    const KEY: u32 = 0xc5acce55;
    itm.lar.write(KEY);

    // ITM: enable the whole peripheral and assign an ID
    let tcr = itm.tcr.read();
    itm.tcr.write({
        // Enables the ITM
        const ITMENA: u32 = 1;
        // The ID of the ITM port. Anything different than 0 will do
        const TRACE_BUS_ID: u32 = 0b1 << 16;
        const TRACE_BUS_ID_MASK: u32 = 0b1111111 << 16;

        ((tcr | ITMENA) & !TRACE_BUS_ID_MASK) | TRACE_BUS_ID
    });

    // ITM: enable the stimulus port 0
    let ter = itm.ter[0].read();
    itm.ter[0].write({
        // Which port
        const N: u32 = 0;

        ter | 1 << N
    });
}

#[doc(hidden)]
pub fn write_fmt(args: Arguments) {
    Port {}.write_fmt(args).ok();
}

#[doc(hidden)]
pub fn write_str(s: &str) {
    Port {}.write_str(s).ok();
}
