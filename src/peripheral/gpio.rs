#[repr(C)]
/// General-purpose I/Os
pub struct Gpio {
    /// 0x00 - GPIO port mode register
    pub moder: Moder,
    /// 0x04 - GPIO port output type register
    pub otyper: Otyper,
    /// 0x08 - GPIO port output speed register
    pub ospeedr: Ospeedr,
    /// 0x0c - GPIO port pull-up/pull-down register
    pub pupdr: Pupdr,
    /// 0x10 - GPIO port input data register
    pub idr: Idr,
    /// 0x14 - GPIO port output data register
    pub odr: Odr,
    /// 0x18 - GPIO port bit set/reset register
    pub bsrr: Bsrr,
    /// 0x1c - GPIO port configuration lock register
    pub lckr: Lckr,
    /// 0x20 - GPIO alternate function low register
    pub afrl: Afrl,
    /// 0x24 - GPIO alternate function high register
    pub afrh: Afrh,
    /// 0x28 - Port bit reset register
    pub brr: Brr,
}

#[repr(C)]
pub struct Moder {
    register: ::volatile_register::RW<u32>,
}

impl Moder {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ModerR, &'w mut ModerW) -> &'w mut ModerW
    {
        let bits = self.register.read();
        let r = ModerR { bits: bits };
        let mut w = ModerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ModerR {
        ModerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ModerW) -> &mut ModerW
    {
        let mut w = ModerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ModerR {
    bits: u32,
}

impl ModerR {
    /// Bits 30:31 - Port x configuration bits (y = 0..15)
    pub fn moder15(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 30;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 28:29 - Port x configuration bits (y = 0..15)
    pub fn moder14(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 26:27 - Port x configuration bits (y = 0..15)
    pub fn moder13(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 26;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 24:25 - Port x configuration bits (y = 0..15)
    pub fn moder12(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 22:23 - Port x configuration bits (y = 0..15)
    pub fn moder11(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 20:21 - Port x configuration bits (y = 0..15)
    pub fn moder10(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 18:19 - Port x configuration bits (y = 0..15)
    pub fn moder9(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 16:17 - Port x configuration bits (y = 0..15)
    pub fn moder8(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 14:15 - Port x configuration bits (y = 0..15)
    pub fn moder7(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 12:13 - Port x configuration bits (y = 0..15)
    pub fn moder6(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 10:11 - Port x configuration bits (y = 0..15)
    pub fn moder5(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:9 - Port x configuration bits (y = 0..15)
    pub fn moder4(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 6:7 - Port x configuration bits (y = 0..15)
    pub fn moder3(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 4:5 - Port x configuration bits (y = 0..15)
    pub fn moder2(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 2:3 - Port x configuration bits (y = 0..15)
    pub fn moder1(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 0:1 - Port x configuration bits (y = 0..15)
    pub fn moder0(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ModerW {
    bits: u32,
}

impl ModerW {
    /// Reset value
    pub fn reset_value() -> Self {
        ModerW { bits: 671088640 }
    }
    /// Bits 30:31 - Port x configuration bits (y = 0..15)
    pub fn moder15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 30;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 28:29 - Port x configuration bits (y = 0..15)
    pub fn moder14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 26:27 - Port x configuration bits (y = 0..15)
    pub fn moder13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 24:25 - Port x configuration bits (y = 0..15)
    pub fn moder12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 22:23 - Port x configuration bits (y = 0..15)
    pub fn moder11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 20:21 - Port x configuration bits (y = 0..15)
    pub fn moder10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 18:19 - Port x configuration bits (y = 0..15)
    pub fn moder9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:17 - Port x configuration bits (y = 0..15)
    pub fn moder8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 14:15 - Port x configuration bits (y = 0..15)
    pub fn moder7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 12:13 - Port x configuration bits (y = 0..15)
    pub fn moder6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 10:11 - Port x configuration bits (y = 0..15)
    pub fn moder5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:9 - Port x configuration bits (y = 0..15)
    pub fn moder4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 6:7 - Port x configuration bits (y = 0..15)
    pub fn moder3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 4:5 - Port x configuration bits (y = 0..15)
    pub fn moder2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 2:3 - Port x configuration bits (y = 0..15)
    pub fn moder1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:1 - Port x configuration bits (y = 0..15)
    pub fn moder0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Otyper {
    register: ::volatile_register::RW<u32>,
}

impl Otyper {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&OtyperR, &'w mut OtyperW) -> &'w mut OtyperW
    {
        let bits = self.register.read();
        let r = OtyperR { bits: bits };
        let mut w = OtyperW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OtyperR {
        OtyperR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OtyperW) -> &mut OtyperW
    {
        let mut w = OtyperW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OtyperR {
    bits: u32,
}

impl OtyperR {
    /// Bit 15 - Port x configuration bits (y = 0..15)
    pub fn ot15(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Port x configuration bits (y = 0..15)
    pub fn ot14(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Port x configuration bits (y = 0..15)
    pub fn ot13(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Port x configuration bits (y = 0..15)
    pub fn ot12(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Port x configuration bits (y = 0..15)
    pub fn ot11(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Port x configuration bits (y = 0..15)
    pub fn ot10(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Port x configuration bits (y = 0..15)
    pub fn ot9(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Port x configuration bits (y = 0..15)
    pub fn ot8(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Port x configuration bits (y = 0..15)
    pub fn ot7(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Port x configuration bits (y = 0..15)
    pub fn ot6(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Port x configuration bits (y = 0..15)
    pub fn ot5(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Port x configuration bits (y = 0..15)
    pub fn ot4(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Port x configuration bits (y = 0..15)
    pub fn ot3(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Port x configuration bits (y = 0..15)
    pub fn ot2(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Port x configuration bits (y = 0..15)
    pub fn ot1(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Port x configuration bits (y = 0..15)
    pub fn ot0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OtyperW {
    bits: u32,
}

impl OtyperW {
    /// Reset value
    pub fn reset_value() -> Self {
        OtyperW { bits: 0 }
    }
    /// Bit 15 - Port x configuration bits (y = 0..15)
    pub fn ot15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Port x configuration bits (y = 0..15)
    pub fn ot14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Port x configuration bits (y = 0..15)
    pub fn ot13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Port x configuration bits (y = 0..15)
    pub fn ot12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Port x configuration bits (y = 0..15)
    pub fn ot11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Port x configuration bits (y = 0..15)
    pub fn ot10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Port x configuration bits (y = 0..15)
    pub fn ot9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Port x configuration bits (y = 0..15)
    pub fn ot8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Port x configuration bits (y = 0..15)
    pub fn ot7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Port x configuration bits (y = 0..15)
    pub fn ot6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Port x configuration bits (y = 0..15)
    pub fn ot5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Port x configuration bits (y = 0..15)
    pub fn ot4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Port x configuration bits (y = 0..15)
    pub fn ot3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Port x configuration bits (y = 0..15)
    pub fn ot2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Port x configuration bits (y = 0..15)
    pub fn ot1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Port x configuration bits (y = 0..15)
    pub fn ot0(&mut self, value: bool) -> &mut Self {
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
pub struct Ospeedr {
    register: ::volatile_register::RW<u32>,
}

impl Ospeedr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&OspeedrR, &'w mut OspeedrW) -> &'w mut OspeedrW
    {
        let bits = self.register.read();
        let r = OspeedrR { bits: bits };
        let mut w = OspeedrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OspeedrR {
        OspeedrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OspeedrW) -> &mut OspeedrW
    {
        let mut w = OspeedrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OspeedrR {
    bits: u32,
}

impl OspeedrR {
    /// Bits 30:31 - Port x configuration bits (y = 0..15)
    pub fn ospeedr15(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 30;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 28:29 - Port x configuration bits (y = 0..15)
    pub fn ospeedr14(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 26:27 - Port x configuration bits (y = 0..15)
    pub fn ospeedr13(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 26;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 24:25 - Port x configuration bits (y = 0..15)
    pub fn ospeedr12(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 22:23 - Port x configuration bits (y = 0..15)
    pub fn ospeedr11(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 20:21 - Port x configuration bits (y = 0..15)
    pub fn ospeedr10(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 18:19 - Port x configuration bits (y = 0..15)
    pub fn ospeedr9(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 16:17 - Port x configuration bits (y = 0..15)
    pub fn ospeedr8(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 14:15 - Port x configuration bits (y = 0..15)
    pub fn ospeedr7(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 12:13 - Port x configuration bits (y = 0..15)
    pub fn ospeedr6(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 10:11 - Port x configuration bits (y = 0..15)
    pub fn ospeedr5(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:9 - Port x configuration bits (y = 0..15)
    pub fn ospeedr4(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 6:7 - Port x configuration bits (y = 0..15)
    pub fn ospeedr3(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 4:5 - Port x configuration bits (y = 0..15)
    pub fn ospeedr2(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 2:3 - Port x configuration bits (y = 0..15)
    pub fn ospeedr1(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 0:1 - Port x configuration bits (y = 0..15)
    pub fn ospeedr0(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OspeedrW {
    bits: u32,
}

impl OspeedrW {
    /// Reset value
    pub fn reset_value() -> Self {
        OspeedrW { bits: 0 }
    }
    /// Bits 30:31 - Port x configuration bits (y = 0..15)
    pub fn ospeedr15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 30;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 28:29 - Port x configuration bits (y = 0..15)
    pub fn ospeedr14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 26:27 - Port x configuration bits (y = 0..15)
    pub fn ospeedr13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 24:25 - Port x configuration bits (y = 0..15)
    pub fn ospeedr12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 22:23 - Port x configuration bits (y = 0..15)
    pub fn ospeedr11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 20:21 - Port x configuration bits (y = 0..15)
    pub fn ospeedr10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 18:19 - Port x configuration bits (y = 0..15)
    pub fn ospeedr9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:17 - Port x configuration bits (y = 0..15)
    pub fn ospeedr8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 14:15 - Port x configuration bits (y = 0..15)
    pub fn ospeedr7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 12:13 - Port x configuration bits (y = 0..15)
    pub fn ospeedr6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 10:11 - Port x configuration bits (y = 0..15)
    pub fn ospeedr5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:9 - Port x configuration bits (y = 0..15)
    pub fn ospeedr4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 6:7 - Port x configuration bits (y = 0..15)
    pub fn ospeedr3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 4:5 - Port x configuration bits (y = 0..15)
    pub fn ospeedr2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 2:3 - Port x configuration bits (y = 0..15)
    pub fn ospeedr1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:1 - Port x configuration bits (y = 0..15)
    pub fn ospeedr0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Pupdr {
    register: ::volatile_register::RW<u32>,
}

impl Pupdr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PupdrR, &'w mut PupdrW) -> &'w mut PupdrW
    {
        let bits = self.register.read();
        let r = PupdrR { bits: bits };
        let mut w = PupdrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PupdrR {
        PupdrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PupdrW) -> &mut PupdrW
    {
        let mut w = PupdrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PupdrR {
    bits: u32,
}

impl PupdrR {
    /// Bits 30:31 - Port x configuration bits (y = 0..15)
    pub fn pupdr15(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 30;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 28:29 - Port x configuration bits (y = 0..15)
    pub fn pupdr14(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 26:27 - Port x configuration bits (y = 0..15)
    pub fn pupdr13(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 26;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 24:25 - Port x configuration bits (y = 0..15)
    pub fn pupdr12(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 22:23 - Port x configuration bits (y = 0..15)
    pub fn pupdr11(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 20:21 - Port x configuration bits (y = 0..15)
    pub fn pupdr10(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 18:19 - Port x configuration bits (y = 0..15)
    pub fn pupdr9(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 16:17 - Port x configuration bits (y = 0..15)
    pub fn pupdr8(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 14:15 - Port x configuration bits (y = 0..15)
    pub fn pupdr7(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 12:13 - Port x configuration bits (y = 0..15)
    pub fn pupdr6(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 10:11 - Port x configuration bits (y = 0..15)
    pub fn pupdr5(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:9 - Port x configuration bits (y = 0..15)
    pub fn pupdr4(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 6:7 - Port x configuration bits (y = 0..15)
    pub fn pupdr3(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 4:5 - Port x configuration bits (y = 0..15)
    pub fn pupdr2(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 2:3 - Port x configuration bits (y = 0..15)
    pub fn pupdr1(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 0:1 - Port x configuration bits (y = 0..15)
    pub fn pupdr0(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PupdrW {
    bits: u32,
}

impl PupdrW {
    /// Reset value
    pub fn reset_value() -> Self {
        PupdrW { bits: 603979776 }
    }
    /// Bits 30:31 - Port x configuration bits (y = 0..15)
    pub fn pupdr15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 30;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 28:29 - Port x configuration bits (y = 0..15)
    pub fn pupdr14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 26:27 - Port x configuration bits (y = 0..15)
    pub fn pupdr13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 24:25 - Port x configuration bits (y = 0..15)
    pub fn pupdr12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 22:23 - Port x configuration bits (y = 0..15)
    pub fn pupdr11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 20:21 - Port x configuration bits (y = 0..15)
    pub fn pupdr10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 18:19 - Port x configuration bits (y = 0..15)
    pub fn pupdr9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:17 - Port x configuration bits (y = 0..15)
    pub fn pupdr8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 14:15 - Port x configuration bits (y = 0..15)
    pub fn pupdr7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 12:13 - Port x configuration bits (y = 0..15)
    pub fn pupdr6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 10:11 - Port x configuration bits (y = 0..15)
    pub fn pupdr5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:9 - Port x configuration bits (y = 0..15)
    pub fn pupdr4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 6:7 - Port x configuration bits (y = 0..15)
    pub fn pupdr3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 4:5 - Port x configuration bits (y = 0..15)
    pub fn pupdr2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 2:3 - Port x configuration bits (y = 0..15)
    pub fn pupdr1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:1 - Port x configuration bits (y = 0..15)
    pub fn pupdr0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Idr {
    register: ::volatile_register::RO<u32>,
}

impl Idr {
    pub fn read(&self) -> IdrR {
        IdrR { bits: self.register.read() }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IdrR {
    bits: u32,
}

impl IdrR {
    /// Bit 15 - Port input data (y = 0..15)
    pub fn idr15(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Port input data (y = 0..15)
    pub fn idr14(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Port input data (y = 0..15)
    pub fn idr13(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Port input data (y = 0..15)
    pub fn idr12(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Port input data (y = 0..15)
    pub fn idr11(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Port input data (y = 0..15)
    pub fn idr10(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Port input data (y = 0..15)
    pub fn idr9(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Port input data (y = 0..15)
    pub fn idr8(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Port input data (y = 0..15)
    pub fn idr7(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Port input data (y = 0..15)
    pub fn idr6(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Port input data (y = 0..15)
    pub fn idr5(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Port input data (y = 0..15)
    pub fn idr4(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Port input data (y = 0..15)
    pub fn idr3(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Port input data (y = 0..15)
    pub fn idr2(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Port input data (y = 0..15)
    pub fn idr1(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Port input data (y = 0..15)
    pub fn idr0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct IdrW {
    bits: u32,
}

impl IdrW {
    /// Reset value
    pub fn reset_value() -> Self {
        IdrW { bits: 0 }
    }
    /// Bit 15 - Port input data (y = 0..15)
    pub fn idr15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Port input data (y = 0..15)
    pub fn idr14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Port input data (y = 0..15)
    pub fn idr13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Port input data (y = 0..15)
    pub fn idr12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Port input data (y = 0..15)
    pub fn idr11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Port input data (y = 0..15)
    pub fn idr10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Port input data (y = 0..15)
    pub fn idr9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Port input data (y = 0..15)
    pub fn idr8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Port input data (y = 0..15)
    pub fn idr7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Port input data (y = 0..15)
    pub fn idr6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Port input data (y = 0..15)
    pub fn idr5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Port input data (y = 0..15)
    pub fn idr4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Port input data (y = 0..15)
    pub fn idr3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Port input data (y = 0..15)
    pub fn idr2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Port input data (y = 0..15)
    pub fn idr1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Port input data (y = 0..15)
    pub fn idr0(&mut self, value: bool) -> &mut Self {
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
pub struct Odr {
    register: ::volatile_register::RW<u32>,
}

impl Odr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&OdrR, &'w mut OdrW) -> &'w mut OdrW
    {
        let bits = self.register.read();
        let r = OdrR { bits: bits };
        let mut w = OdrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OdrR {
        OdrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OdrW) -> &mut OdrW
    {
        let mut w = OdrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OdrR {
    bits: u32,
}

impl OdrR {
    /// Bit 15 - Port output data (y = 0..15)
    pub fn odr15(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Port output data (y = 0..15)
    pub fn odr14(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Port output data (y = 0..15)
    pub fn odr13(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Port output data (y = 0..15)
    pub fn odr12(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Port output data (y = 0..15)
    pub fn odr11(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Port output data (y = 0..15)
    pub fn odr10(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Port output data (y = 0..15)
    pub fn odr9(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Port output data (y = 0..15)
    pub fn odr8(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Port output data (y = 0..15)
    pub fn odr7(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Port output data (y = 0..15)
    pub fn odr6(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Port output data (y = 0..15)
    pub fn odr5(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Port output data (y = 0..15)
    pub fn odr4(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Port output data (y = 0..15)
    pub fn odr3(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Port output data (y = 0..15)
    pub fn odr2(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Port output data (y = 0..15)
    pub fn odr1(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Port output data (y = 0..15)
    pub fn odr0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct OdrW {
    bits: u32,
}

impl OdrW {
    /// Reset value
    pub fn reset_value() -> Self {
        OdrW { bits: 0 }
    }
    /// Bit 15 - Port output data (y = 0..15)
    pub fn odr15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Port output data (y = 0..15)
    pub fn odr14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Port output data (y = 0..15)
    pub fn odr13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Port output data (y = 0..15)
    pub fn odr12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Port output data (y = 0..15)
    pub fn odr11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Port output data (y = 0..15)
    pub fn odr10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Port output data (y = 0..15)
    pub fn odr9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Port output data (y = 0..15)
    pub fn odr8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Port output data (y = 0..15)
    pub fn odr7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Port output data (y = 0..15)
    pub fn odr6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Port output data (y = 0..15)
    pub fn odr5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Port output data (y = 0..15)
    pub fn odr4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Port output data (y = 0..15)
    pub fn odr3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Port output data (y = 0..15)
    pub fn odr2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Port output data (y = 0..15)
    pub fn odr1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Port output data (y = 0..15)
    pub fn odr0(&mut self, value: bool) -> &mut Self {
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
pub struct Bsrr {
    register: ::volatile_register::WO<u32>,
}

impl Bsrr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut BsrrW) -> &mut BsrrW
    {
        let mut w = BsrrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BsrrR {
    bits: u32,
}

impl BsrrR {
    /// Bit 31 - Port x reset bit y (y = 0..15)
    pub fn br15(&self) -> bool {
        const OFFSET: u8 = 31;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 30 - Port x reset bit y (y = 0..15)
    pub fn br14(&self) -> bool {
        const OFFSET: u8 = 30;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 29 - Port x reset bit y (y = 0..15)
    pub fn br13(&self) -> bool {
        const OFFSET: u8 = 29;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 28 - Port x reset bit y (y = 0..15)
    pub fn br12(&self) -> bool {
        const OFFSET: u8 = 28;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 27 - Port x reset bit y (y = 0..15)
    pub fn br11(&self) -> bool {
        const OFFSET: u8 = 27;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 26 - Port x reset bit y (y = 0..15)
    pub fn br10(&self) -> bool {
        const OFFSET: u8 = 26;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 25 - Port x reset bit y (y = 0..15)
    pub fn br9(&self) -> bool {
        const OFFSET: u8 = 25;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 24 - Port x reset bit y (y = 0..15)
    pub fn br8(&self) -> bool {
        const OFFSET: u8 = 24;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 23 - Port x reset bit y (y = 0..15)
    pub fn br7(&self) -> bool {
        const OFFSET: u8 = 23;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 22 - Port x reset bit y (y = 0..15)
    pub fn br6(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - Port x reset bit y (y = 0..15)
    pub fn br5(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 20 - Port x reset bit y (y = 0..15)
    pub fn br4(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - Port x reset bit y (y = 0..15)
    pub fn br3(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - Port x reset bit y (y = 0..15)
    pub fn br2(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - Port x reset bit y (y = 0..15)
    pub fn br1(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 16 - Port x set bit y (y= 0..15)
    pub fn br0(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Port x set bit y (y= 0..15)
    pub fn bs15(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Port x set bit y (y= 0..15)
    pub fn bs14(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Port x set bit y (y= 0..15)
    pub fn bs13(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Port x set bit y (y= 0..15)
    pub fn bs12(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Port x set bit y (y= 0..15)
    pub fn bs11(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Port x set bit y (y= 0..15)
    pub fn bs10(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Port x set bit y (y= 0..15)
    pub fn bs9(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Port x set bit y (y= 0..15)
    pub fn bs8(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Port x set bit y (y= 0..15)
    pub fn bs7(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Port x set bit y (y= 0..15)
    pub fn bs6(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Port x set bit y (y= 0..15)
    pub fn bs5(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Port x set bit y (y= 0..15)
    pub fn bs4(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Port x set bit y (y= 0..15)
    pub fn bs3(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Port x set bit y (y= 0..15)
    pub fn bs2(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Port x set bit y (y= 0..15)
    pub fn bs1(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Port x set bit y (y= 0..15)
    pub fn bs0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BsrrW {
    bits: u32,
}

impl BsrrW {
    /// Reset value
    pub fn reset_value() -> Self {
        BsrrW { bits: 0 }
    }
    /// Bit 31 - Port x reset bit y (y = 0..15)
    pub fn br15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 30 - Port x reset bit y (y = 0..15)
    pub fn br14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 29 - Port x reset bit y (y = 0..15)
    pub fn br13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 28 - Port x reset bit y (y = 0..15)
    pub fn br12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 27 - Port x reset bit y (y = 0..15)
    pub fn br11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 26 - Port x reset bit y (y = 0..15)
    pub fn br10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 25 - Port x reset bit y (y = 0..15)
    pub fn br9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 24 - Port x reset bit y (y = 0..15)
    pub fn br8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 23 - Port x reset bit y (y = 0..15)
    pub fn br7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 22 - Port x reset bit y (y = 0..15)
    pub fn br6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - Port x reset bit y (y = 0..15)
    pub fn br5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 20 - Port x reset bit y (y = 0..15)
    pub fn br4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - Port x reset bit y (y = 0..15)
    pub fn br3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - Port x reset bit y (y = 0..15)
    pub fn br2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - Port x reset bit y (y = 0..15)
    pub fn br1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - Port x set bit y (y= 0..15)
    pub fn br0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - Port x set bit y (y= 0..15)
    pub fn bs15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Port x set bit y (y= 0..15)
    pub fn bs14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Port x set bit y (y= 0..15)
    pub fn bs13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Port x set bit y (y= 0..15)
    pub fn bs12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Port x set bit y (y= 0..15)
    pub fn bs11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Port x set bit y (y= 0..15)
    pub fn bs10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Port x set bit y (y= 0..15)
    pub fn bs9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Port x set bit y (y= 0..15)
    pub fn bs8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Port x set bit y (y= 0..15)
    pub fn bs7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Port x set bit y (y= 0..15)
    pub fn bs6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Port x set bit y (y= 0..15)
    pub fn bs5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Port x set bit y (y= 0..15)
    pub fn bs4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Port x set bit y (y= 0..15)
    pub fn bs3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Port x set bit y (y= 0..15)
    pub fn bs2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Port x set bit y (y= 0..15)
    pub fn bs1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Port x set bit y (y= 0..15)
    pub fn bs0(&mut self, value: bool) -> &mut Self {
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
pub struct Lckr {
    register: ::volatile_register::RW<u32>,
}

impl Lckr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&LckrR, &'w mut LckrW) -> &'w mut LckrW
    {
        let bits = self.register.read();
        let r = LckrR { bits: bits };
        let mut w = LckrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> LckrR {
        LckrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut LckrW) -> &mut LckrW
    {
        let mut w = LckrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LckrR {
    bits: u32,
}

impl LckrR {
    /// Bit 16 - Lok Key
    pub fn lckk(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Port x lock bit y (y= 0..15)
    pub fn lck15(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Port x lock bit y (y= 0..15)
    pub fn lck14(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Port x lock bit y (y= 0..15)
    pub fn lck13(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Port x lock bit y (y= 0..15)
    pub fn lck12(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Port x lock bit y (y= 0..15)
    pub fn lck11(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Port x lock bit y (y= 0..15)
    pub fn lck10(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Port x lock bit y (y= 0..15)
    pub fn lck9(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Port x lock bit y (y= 0..15)
    pub fn lck8(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Port x lock bit y (y= 0..15)
    pub fn lck7(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Port x lock bit y (y= 0..15)
    pub fn lck6(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Port x lock bit y (y= 0..15)
    pub fn lck5(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Port x lock bit y (y= 0..15)
    pub fn lck4(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Port x lock bit y (y= 0..15)
    pub fn lck3(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Port x lock bit y (y= 0..15)
    pub fn lck2(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Port x lock bit y (y= 0..15)
    pub fn lck1(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Port x lock bit y (y= 0..15)
    pub fn lck0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LckrW {
    bits: u32,
}

impl LckrW {
    /// Reset value
    pub fn reset_value() -> Self {
        LckrW { bits: 0 }
    }
    /// Bit 16 - Lok Key
    pub fn lckk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - Port x lock bit y (y= 0..15)
    pub fn lck15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Port x lock bit y (y= 0..15)
    pub fn lck14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Port x lock bit y (y= 0..15)
    pub fn lck13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Port x lock bit y (y= 0..15)
    pub fn lck12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Port x lock bit y (y= 0..15)
    pub fn lck11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Port x lock bit y (y= 0..15)
    pub fn lck10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Port x lock bit y (y= 0..15)
    pub fn lck9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Port x lock bit y (y= 0..15)
    pub fn lck8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Port x lock bit y (y= 0..15)
    pub fn lck7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Port x lock bit y (y= 0..15)
    pub fn lck6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Port x lock bit y (y= 0..15)
    pub fn lck5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Port x lock bit y (y= 0..15)
    pub fn lck4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Port x lock bit y (y= 0..15)
    pub fn lck3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Port x lock bit y (y= 0..15)
    pub fn lck2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Port x lock bit y (y= 0..15)
    pub fn lck1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Port x lock bit y (y= 0..15)
    pub fn lck0(&mut self, value: bool) -> &mut Self {
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
pub struct Afrl {
    register: ::volatile_register::RW<u32>,
}

impl Afrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AfrlR, &'w mut AfrlW) -> &'w mut AfrlW
    {
        let bits = self.register.read();
        let r = AfrlR { bits: bits };
        let mut w = AfrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AfrlR {
        AfrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AfrlW) -> &mut AfrlW
    {
        let mut w = AfrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct AfrlR {
    bits: u32,
}

impl AfrlR {
    /// Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl7(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 28;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl6(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl5(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl4(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl3(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl2(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl1(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl0(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct AfrlW {
    bits: u32,
}

impl AfrlW {
    /// Reset value
    pub fn reset_value() -> Self {
        AfrlW { bits: 0 }
    }
    /// Bits 28:31 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 24:27 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 20:23 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:19 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 12:15 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:11 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 4:7 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:3 - Alternate function selection for port x bit y (y = 0..7)
    pub fn afrl0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Afrh {
    register: ::volatile_register::RW<u32>,
}

impl Afrh {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AfrhR, &'w mut AfrhW) -> &'w mut AfrhW
    {
        let bits = self.register.read();
        let r = AfrhR { bits: bits };
        let mut w = AfrhW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AfrhR {
        AfrhR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AfrhW) -> &mut AfrhW
    {
        let mut w = AfrhW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct AfrhR {
    bits: u32,
}

impl AfrhR {
    /// Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh15(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 28;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh14(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh13(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh12(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh11(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh10(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh9(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh8(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct AfrhW {
    bits: u32,
}

impl AfrhW {
    /// Reset value
    pub fn reset_value() -> Self {
        AfrhW { bits: 0 }
    }
    /// Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    pub fn afrh8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

#[repr(C)]
pub struct Brr {
    register: ::volatile_register::WO<u32>,
}

impl Brr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut BrrW) -> &mut BrrW
    {
        let mut w = BrrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrrR {
    bits: u32,
}

impl BrrR {
    /// Bit 0 - Port x Reset bit y
    pub fn br0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Port x Reset bit y
    pub fn br1(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Port x Reset bit y
    pub fn br2(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Port x Reset bit y
    pub fn br3(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Port x Reset bit y
    pub fn br4(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Port x Reset bit y
    pub fn br5(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Port x Reset bit y
    pub fn br6(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Port x Reset bit y
    pub fn br7(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Port x Reset bit y
    pub fn br8(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Port x Reset bit y
    pub fn br9(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Port x Reset bit y
    pub fn br10(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Port x Reset bit y
    pub fn br11(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Port x Reset bit y
    pub fn br12(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Port x Reset bit y
    pub fn br13(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Port x Reset bit y
    pub fn br14(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Port x Reset bit y
    pub fn br15(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BrrW {
    bits: u32,
}

impl BrrW {
    /// Reset value
    pub fn reset_value() -> Self {
        BrrW { bits: 0 }
    }
    /// Bit 0 - Port x Reset bit y
    pub fn br0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Port x Reset bit y
    pub fn br1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Port x Reset bit y
    pub fn br2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Port x Reset bit y
    pub fn br3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Port x Reset bit y
    pub fn br4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Port x Reset bit y
    pub fn br5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Port x Reset bit y
    pub fn br6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Port x Reset bit y
    pub fn br7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Port x Reset bit y
    pub fn br8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Port x Reset bit y
    pub fn br9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Port x Reset bit y
    pub fn br10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Port x Reset bit y
    pub fn br11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Port x Reset bit y
    pub fn br12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Port x Reset bit y
    pub fn br13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Port x Reset bit y
    pub fn br14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - Port x Reset bit y
    pub fn br15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
