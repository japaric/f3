#[repr(C)]
/// Basic timers
pub struct Tim {
    /// 0x00 - control register 1
    pub cr1: Cr1,
    /// 0x04 - control register 2
    pub cr2: Cr2,
    reserved0: [u8; 4usize],
    /// 0x0c - DMA/Interrupt enable register
    pub dier: Dier,
    /// 0x10 - status register
    pub sr: Sr,
    /// 0x14 - event generation register
    pub egr: Egr,
    reserved1: [u8; 12usize],
    /// 0x24 - counter
    pub cnt: Cnt,
    /// 0x28 - prescaler
    pub psc: Psc,
    /// 0x2c - auto-reload register
    pub arr: Arr,
}

#[repr(C)]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr1R, &'w mut Cr1W) -> &'w mut Cr1W
    {
        let bits = self.register.read();
        let r = Cr1R { bits: bits };
        let mut w = Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr1R {
        Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr1W) -> &mut Cr1W
    {
        let mut w = Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Cr1R {
    bits: u32,
}

impl Cr1R {
    /// Bit 0 - Counter enable
    pub fn cen(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Update disable
    pub fn udis(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Update request source
    pub fn urs(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - One-pulse mode
    pub fn opm(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Auto-reload preload enable
    pub fn arpe(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - UIF status bit remapping
    pub fn uifremap(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Cr1W {
    bits: u32,
}

impl Cr1W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cr1W { bits: 0 }
    }
    /// Bit 0 - Counter enable
    pub fn cen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Update disable
    pub fn udis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Update request source
    pub fn urs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - One-pulse mode
    pub fn opm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Auto-reload preload enable
    pub fn arpe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - UIF status bit remapping
    pub fn uifremap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

#[repr(C)]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Cr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr2R, &'w mut Cr2W) -> &'w mut Cr2W
    {
        let bits = self.register.read();
        let r = Cr2R { bits: bits };
        let mut w = Cr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr2R {
        Cr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr2W) -> &mut Cr2W
    {
        let mut w = Cr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Cr2R {
    bits: u32,
}

impl Cr2R {
    /// Bits 4:6 - Master mode selection
    pub fn mms(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Cr2W {
    bits: u32,
}

impl Cr2W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cr2W { bits: 0 }
    }
    /// Bits 4:6 - Master mode selection
    pub fn mms(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Dier {
    register: ::volatile_register::RW<u32>,
}

impl Dier {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DierR, &'w mut DierW) -> &'w mut DierW
    {
        let bits = self.register.read();
        let r = DierR { bits: bits };
        let mut w = DierW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DierR {
        DierR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DierW) -> &mut DierW
    {
        let mut w = DierW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DierR {
    bits: u32,
}

impl DierR {
    /// Bit 8 - Update DMA request enable
    pub fn ude(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Update interrupt enable
    pub fn uie(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct DierW {
    bits: u32,
}

impl DierW {
    /// Reset value
    pub fn reset_value() -> Self {
        DierW { bits: 0 }
    }
    /// Bit 8 - Update DMA request enable
    pub fn ude(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Update interrupt enable
    pub fn uie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

#[repr(C)]
pub struct Sr {
    register: ::volatile_register::RW<u32>,
}

impl Sr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&SrR, &'w mut SrW) -> &'w mut SrW
    {
        let bits = self.register.read();
        let r = SrR { bits: bits };
        let mut w = SrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SrR {
        SrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SrW) -> &mut SrW
    {
        let mut w = SrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SrR {
    bits: u32,
}

impl SrR {
    /// Bit 0 - Update interrupt flag
    pub fn uif(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SrW {
    bits: u32,
}

impl SrW {
    /// Reset value
    pub fn reset_value() -> Self {
        SrW { bits: 0 }
    }
    /// Bit 0 - Update interrupt flag
    pub fn uif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

#[repr(C)]
pub struct Egr {
    register: ::volatile_register::WO<u32>,
}

impl Egr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut EgrW) -> &mut EgrW
    {
        let mut w = EgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EgrR {
    bits: u32,
}

impl EgrR {
    /// Bit 0 - Update generation
    pub fn ug(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct EgrW {
    bits: u32,
}

impl EgrW {
    /// Reset value
    pub fn reset_value() -> Self {
        EgrW { bits: 0 }
    }
    /// Bit 0 - Update generation
    pub fn ug(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

#[repr(C)]
pub struct Cnt {
    register: ::volatile_register::RW<u32>,
}

impl Cnt {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CntR, &'w mut CntW) -> &'w mut CntW
    {
        let bits = self.register.read();
        let r = CntR { bits: bits };
        let mut w = CntW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CntR {
        CntR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CntW) -> &mut CntW
    {
        let mut w = CntW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CntR {
    bits: u32,
}

impl CntR {
    /// Bits 0:15 - Low counter value
    pub fn cnt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    /// Bit 31 - UIF Copy
    pub fn uifcpy(&self) -> bool {
        const OFFSET: u8 = 31;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct CntW {
    bits: u32,
}

impl CntW {
    /// Reset value
    pub fn reset_value() -> Self {
        CntW { bits: 0 }
    }
    /// Bits 0:15 - Low counter value
    pub fn cnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Psc {
    register: ::volatile_register::RW<u32>,
}

impl Psc {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PscR, &'w mut PscW) -> &'w mut PscW
    {
        let bits = self.register.read();
        let r = PscR { bits: bits };
        let mut w = PscW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PscR {
        PscR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PscW) -> &mut PscW
    {
        let mut w = PscW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PscR {
    bits: u32,
}

impl PscR {
    /// Bits 0:15 - Prescaler value
    pub fn psc(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PscW {
    bits: u32,
}

impl PscW {
    /// Reset value
    pub fn reset_value() -> Self {
        PscW { bits: 0 }
    }
    /// Bits 0:15 - Prescaler value
    pub fn psc(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Arr {
    register: ::volatile_register::RW<u32>,
}

impl Arr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ArrR, &'w mut ArrW) -> &'w mut ArrW
    {
        let bits = self.register.read();
        let r = ArrR { bits: bits };
        let mut w = ArrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ArrR {
        ArrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ArrW) -> &mut ArrW
    {
        let mut w = ArrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ArrR {
    bits: u32,
}

impl ArrR {
    /// Bits 0:15 - Low Auto-reload value
    pub fn arr(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ArrW {
    bits: u32,
}

impl ArrW {
    /// Reset value
    pub fn reset_value() -> Self {
        ArrW { bits: 0 }
    }
    /// Bits 0:15 - Low Auto-reload value
    pub fn arr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
