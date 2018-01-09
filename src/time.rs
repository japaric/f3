#[derive(Clone, Copy)]
pub struct Bps(pub u32);

#[derive(Clone, Copy)]
pub struct Hertz(pub u32);

#[derive(Clone, Copy)]
pub struct KiloHertz(pub u32);

#[derive(Clone, Copy)]
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

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}
