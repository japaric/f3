use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;

use rcc::Clocks;

pub struct Delay {
    clocks: Clocks,
    syst: SYST,
}

impl Delay {
    pub fn new(syst: SYST, clocks: Clocks) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay { syst, clocks }
    }

    pub fn ms(&mut self, ms: u32) {
        let rvr = ms * (self.clocks.sysclk().0 / 1_000);

        assert!(rvr < (1 << 24));

        self.syst.set_reload(rvr);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }

    pub fn us(&mut self, us: u32) {
        let rvr = us * (self.clocks.sysclk().0 / 1_000_000);

        assert!(rvr < (1 << 24));

        self.syst.set_reload(rvr);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }

    pub fn free(self) -> SYST {
        self.syst
    }
}
