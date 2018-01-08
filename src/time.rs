pub struct Bps(pub u32);
pub struct Hertz(pub u32);
pub struct KiloHertz(pub u32);
pub struct MegaHertz(pub u32);

pub trait U32Ext {
    fn bps(self) -> Bps;
    fn hz(self) -> Hertz;
    fn khz(self) -> KiloHertz;
    fn mhz(self) -> MegaHertz;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }
}
