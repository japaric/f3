use cast::u32;

use cortex_m::peripheral::SYST;
use cortex_m::peripheral::syst::SystClkSource;

pub struct Delay {
    syst: SYST,
}

impl Delay {
    pub fn new(syst: SYST) -> Self {
        syst.set_clock_source(SystClkSource::Core);

        Delay { syst }
    }

    pub fn ms(&mut self, ms: u16) {
        self.syst.set_reload(u32(ms) * 8_000);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }

    pub fn us(&mut self, us: u32) {
        self.syst.set_reload(us * 8);
        self.syst.clear_current();
        self.syst.enable_counter();

        while !self.syst.has_wrapped() {}

        self.syst.disable_counter();
    }

    pub fn free(self) -> SYST {
        self.syst
    }
}
