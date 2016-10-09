#[repr(C)]
/// Inter-integrated circuit
pub struct I2c {
    /// 0x00 - Control register 1
    pub cr1: Cr1,
    /// 0x04 - Control register 2
    pub cr2: Cr2,
    /// 0x08 - Own address register 1
    pub oar1: Oar1,
    /// 0x0c - Own address register 2
    pub oar2: Oar2,
    /// 0x10 - Timing register
    pub timingr: Timingr,
    /// 0x14 - Status register 1
    pub timeoutr: Timeoutr,
    /// 0x18 - Interrupt and Status register
    pub isr: Isr,
    /// 0x1c - Interrupt clear register
    pub icr: Icr,
    /// 0x20 - PEC register
    pub pecr: Pecr,
    /// 0x24 - Receive data register
    pub rxdr: Rxdr,
    /// 0x28 - Transmit data register
    pub txdr: Txdr,
}

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
pub struct Cr1R {
    bits: u32,
}

impl Cr1R {
    /// Bit 0 - Peripheral enable
    pub fn pe(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - TX Interrupt enable
    pub fn txie(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - RX Interrupt enable
    pub fn rxie(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Address match interrupt enable (slave only)
    pub fn addrie(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Not acknowledge received interrupt enable
    pub fn nackie(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - STOP detection Interrupt enable
    pub fn stopie(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Transfer Complete interrupt enable
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Error interrupts enable
    pub fn errie(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 8:11 - Digital noise filter
    pub fn dnf(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 12 - Analog noise filter OFF
    pub fn anfoff(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - DMA transmission requests enable
    pub fn txdmaen(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - DMA reception requests enable
    pub fn rxdmaen(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 16 - Slave byte control
    pub fn sbc(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - Clock stretching disable
    pub fn nostretch(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - Wakeup from STOP enable
    pub fn wupen(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - General call enable
    pub fn gcen(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 20 - SMBus Host address enable
    pub fn smbhen(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - SMBus Device Default address enable
    pub fn smbden(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 22 - SMBUS alert enable
    pub fn alerten(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 23 - PEC enable
    pub fn pecen(&self) -> bool {
        const OFFSET: u8 = 23;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Cr1W {
    bits: u32,
}

impl Cr1W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cr1W { bits: 0 }
    }
    /// Bit 0 - Peripheral enable
    pub fn pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - TX Interrupt enable
    pub fn txie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - RX Interrupt enable
    pub fn rxie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Address match interrupt enable (slave only)
    pub fn addrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Not acknowledge received interrupt enable
    pub fn nackie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - STOP detection Interrupt enable
    pub fn stopie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Transfer Complete interrupt enable
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Error interrupts enable
    pub fn errie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 8:11 - Digital noise filter
    pub fn dnf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 12 - Analog noise filter OFF
    pub fn anfoff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Software reset
    pub fn swrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - DMA transmission requests enable
    pub fn txdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - DMA reception requests enable
    pub fn rxdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - Slave byte control
    pub fn sbc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - Clock stretching disable
    pub fn nostretch(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - Wakeup from STOP enable
    pub fn wupen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - General call enable
    pub fn gcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 20 - SMBus Host address enable
    pub fn smbhen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - SMBus Device Default address enable
    pub fn smbden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 22 - SMBUS alert enable
    pub fn alerten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 23 - PEC enable
    pub fn pecen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

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
pub struct Cr2R {
    bits: u32,
}

impl Cr2R {
    /// Bit 26 - Packet error checking byte
    pub fn pecbyte(&self) -> bool {
        const OFFSET: u8 = 26;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 25 - Automatic end mode (master mode)
    pub fn autoend(&self) -> bool {
        const OFFSET: u8 = 25;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 24 - NBYTES reload mode
    pub fn reload(&self) -> bool {
        const OFFSET: u8 = 24;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 16:23 - Number of bytes
    pub fn nbytes(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 15 - NACK generation (slave mode)
    pub fn nack(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Stop generation (master mode)
    pub fn stop(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Start generation
    pub fn start(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - 10-bit address header only read direction (master receiver mode)
    pub fn head10r(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - 10-bit addressing mode (master mode)
    pub fn add10(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Transfer direction (master mode)
    pub fn rd_wrn(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 8:9 - Slave address bit 9:8 (master mode)
    pub fn sadd8(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 1:7 - Slave address bit 7:1 (master mode)
    pub fn sadd1(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 1;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 0 - Slave address bit 0 (master mode)
    pub fn sadd0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Cr2W {
    bits: u32,
}

impl Cr2W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cr2W { bits: 0 }
    }
    /// Bit 26 - Packet error checking byte
    pub fn pecbyte(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 25 - Automatic end mode (master mode)
    pub fn autoend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 24 - NBYTES reload mode
    pub fn reload(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 16:23 - Number of bytes
    pub fn nbytes(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 15 - NACK generation (slave mode)
    pub fn nack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Stop generation (master mode)
    pub fn stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Start generation
    pub fn start(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - 10-bit address header only read direction (master receiver mode)
    pub fn head10r(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - 10-bit addressing mode (master mode)
    pub fn add10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Transfer direction (master mode)
    pub fn rd_wrn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 8:9 - Slave address bit 9:8 (master mode)
    pub fn sadd8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 1:7 - Slave address bit 7:1 (master mode)
    pub fn sadd1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 0 - Slave address bit 0 (master mode)
    pub fn sadd0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Oar1 {
    register: ::volatile_register::RW<u32>,
}

impl Oar1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Oar1R, &'w mut Oar1W) -> &'w mut Oar1W
    {
        let bits = self.register.read();
        let r = Oar1R { bits: bits };
        let mut w = Oar1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Oar1R {
        Oar1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Oar1W) -> &mut Oar1W
    {
        let mut w = Oar1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
pub struct Oar1R {
    bits: u32,
}

impl Oar1R {
    /// Bit 0 - Interface address
    pub fn oa1_0(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 1:7 - Interface address
    pub fn oa1_1(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 1;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:9 - Interface address
    pub fn oa1_8(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 10 - Own Address 1 10-bit mode
    pub fn oa1mode(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Own Address 1 enable
    pub fn oa1en(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Oar1W {
    bits: u32,
}

impl Oar1W {
    /// Reset value
    pub fn reset_value() -> Self {
        Oar1W { bits: 0 }
    }
    /// Bit 0 - Interface address
    pub fn oa1_0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 1:7 - Interface address
    pub fn oa1_1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:9 - Interface address
    pub fn oa1_8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 10 - Own Address 1 10-bit mode
    pub fn oa1mode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - Own Address 1 enable
    pub fn oa1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Oar2 {
    register: ::volatile_register::RW<u32>,
}

impl Oar2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Oar2R, &'w mut Oar2W) -> &'w mut Oar2W
    {
        let bits = self.register.read();
        let r = Oar2R { bits: bits };
        let mut w = Oar2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Oar2R {
        Oar2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Oar2W) -> &mut Oar2W
    {
        let mut w = Oar2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
pub struct Oar2R {
    bits: u32,
}

impl Oar2R {
    /// Bits 1:7 - Interface address
    pub fn oa2(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 1;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:10 - Own Address 2 masks
    pub fn oa2msk(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 15 - Own Address 2 enable
    pub fn oa2en(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Oar2W {
    bits: u32,
}

impl Oar2W {
    /// Reset value
    pub fn reset_value() -> Self {
        Oar2W { bits: 0 }
    }
    /// Bits 1:7 - Interface address
    pub fn oa2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:10 - Own Address 2 masks
    pub fn oa2msk(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 15 - Own Address 2 enable
    pub fn oa2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Timingr {
    register: ::volatile_register::RW<u32>,
}

impl Timingr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&TimingrR, &'w mut TimingrW) -> &'w mut TimingrW
    {
        let bits = self.register.read();
        let r = TimingrR { bits: bits };
        let mut w = TimingrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TimingrR {
        TimingrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TimingrW) -> &mut TimingrW
    {
        let mut w = TimingrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
pub struct TimingrR {
    bits: u32,
}

impl TimingrR {
    /// Bits 0:7 - SCL low period (master mode)
    pub fn scll(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:15 - SCL high period (master mode)
    pub fn sclh(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 16:19 - Data hold time
    pub fn sdadel(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 20:23 - Data setup time
    pub fn scldel(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 28:31 - Timing prescaler
    pub fn presc(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 28;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct TimingrW {
    bits: u32,
}

impl TimingrW {
    /// Reset value
    pub fn reset_value() -> Self {
        TimingrW { bits: 0 }
    }
    /// Bits 0:7 - SCL low period (master mode)
    pub fn scll(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:15 - SCL high period (master mode)
    pub fn sclh(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:19 - Data hold time
    pub fn sdadel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 20:23 - Data setup time
    pub fn scldel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 28:31 - Timing prescaler
    pub fn presc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Timeoutr {
    register: ::volatile_register::RW<u32>,
}

impl Timeoutr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&TimeoutrR, &'w mut TimeoutrW)
                                -> &'w mut TimeoutrW
    {
        let bits = self.register.read();
        let r = TimeoutrR { bits: bits };
        let mut w = TimeoutrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TimeoutrR {
        TimeoutrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TimeoutrW) -> &mut TimeoutrW
    {
        let mut w = TimeoutrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
pub struct TimeoutrR {
    bits: u32,
}

impl TimeoutrR {
    /// Bits 0:11 - Bus timeout A
    pub fn timeouta(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    /// Bit 12 - Idle clock timeout detection
    pub fn tidle(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Clock timeout enable
    pub fn timouten(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 16:27 - Bus timeout B
    pub fn timeoutb(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    /// Bit 31 - Extended clock timeout enable
    pub fn texten(&self) -> bool {
        const OFFSET: u8 = 31;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct TimeoutrW {
    bits: u32,
}

impl TimeoutrW {
    /// Reset value
    pub fn reset_value() -> Self {
        TimeoutrW { bits: 0 }
    }
    /// Bits 0:11 - Bus timeout A
    pub fn timeouta(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 12 - Idle clock timeout detection
    pub fn tidle(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - Clock timeout enable
    pub fn timouten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 16:27 - Bus timeout B
    pub fn timeoutb(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 31 - Extended clock timeout enable
    pub fn texten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Isr {
    register: ::volatile_register::RW<u32>,
}

impl Isr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IsrR, &'w mut IsrW) -> &'w mut IsrW
    {
        let bits = self.register.read();
        let r = IsrR { bits: bits };
        let mut w = IsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IsrR {
        IsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IsrW) -> &mut IsrW
    {
        let mut w = IsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
pub struct IsrR {
    bits: u32,
}

impl IsrR {
    /// Bits 17:23 - Address match code (Slave mode)
    pub fn addcode(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 17;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 16 - Transfer direction (Slave mode)
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Bus busy
    pub fn busy(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - SMBus alert
    pub fn alert(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Timeout or t_low detection flag
    pub fn timeout(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - PEC Error in reception
    pub fn pecerr(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Overrun/Underrun (slave mode)
    pub fn ovr(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Arbitration lost
    pub fn arlo(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Bus error
    pub fn berr(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Transfer Complete Reload
    pub fn tcr(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Transfer Complete (master mode)
    pub fn tc(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Stop detection flag
    pub fn stopf(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Not acknowledge received flag
    pub fn nackf(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Address matched (slave mode)
    pub fn addr(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Receive data register not empty (receivers)
    pub fn rxne(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Transmit interrupt status (transmitters)
    pub fn txis(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Transmit data register empty (transmitters)
    pub fn txe(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct IsrW {
    bits: u32,
}

impl IsrW {
    /// Reset value
    pub fn reset_value() -> Self {
        IsrW { bits: 1 }
    }
    /// Bit 1 - Transmit interrupt status (transmitters)
    pub fn txis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Transmit data register empty (transmitters)
    pub fn txe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Icr {
    register: ::volatile_register::WO<u32>,
}

impl Icr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut IcrW) -> &mut IcrW
    {
        let mut w = IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
pub struct IcrR {
    bits: u32,
}

impl IcrR {
    /// Bit 13 - Alert flag clear
    pub fn alertcf(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Timeout detection flag clear
    pub fn timoutcf(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - PEC Error flag clear
    pub fn peccf(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Overrun/Underrun flag clear
    pub fn ovrcf(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Arbitration lost flag clear
    pub fn arlocf(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Bus error flag clear
    pub fn berrcf(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Stop detection flag clear
    pub fn stopcf(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Not Acknowledge flag clear
    pub fn nackcf(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Address Matched flag clear
    pub fn addrcf(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct IcrW {
    bits: u32,
}

impl IcrW {
    /// Reset value
    pub fn reset_value() -> Self {
        IcrW { bits: 0 }
    }
    /// Bit 13 - Alert flag clear
    pub fn alertcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Timeout detection flag clear
    pub fn timoutcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - PEC Error flag clear
    pub fn peccf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Overrun/Underrun flag clear
    pub fn ovrcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Arbitration lost flag clear
    pub fn arlocf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Bus error flag clear
    pub fn berrcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Stop detection flag clear
    pub fn stopcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Not Acknowledge flag clear
    pub fn nackcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Address Matched flag clear
    pub fn addrcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Pecr {
    register: ::volatile_register::RO<u32>,
}

impl Pecr {
    pub fn read(&self) -> PecrR {
        PecrR { bits: self.register.read() }
    }
}

#[derive(Clone, Copy)]
pub struct PecrR {
    bits: u32,
}

impl PecrR {
    /// Bits 0:7 - Packet error checking register
    pub fn pec(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct PecrW {
    bits: u32,
}

impl PecrW {
    /// Reset value
    pub fn reset_value() -> Self {
        PecrW { bits: 0 }
    }
    /// Bits 0:7 - Packet error checking register
    pub fn pec(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Rxdr {
    register: ::volatile_register::RO<u32>,
}

impl Rxdr {
    pub fn read(&self) -> RxdrR {
        RxdrR { bits: self.register.read() }
    }
}

#[derive(Clone, Copy)]
pub struct RxdrR {
    bits: u32,
}

impl RxdrR {
    /// Bits 0:7 - 8-bit receive data
    pub fn rxdata(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct RxdrW {
    bits: u32,
}

impl RxdrW {
    /// Reset value
    pub fn reset_value() -> Self {
        RxdrW { bits: 0 }
    }
    /// Bits 0:7 - 8-bit receive data
    pub fn rxdata(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Txdr {
    register: ::volatile_register::RW<u32>,
}

impl Txdr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&TxdrR, &'w mut TxdrW) -> &'w mut TxdrW
    {
        let bits = self.register.read();
        let r = TxdrR { bits: bits };
        let mut w = TxdrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> TxdrR {
        TxdrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut TxdrW) -> &mut TxdrW
    {
        let mut w = TxdrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

#[derive(Clone, Copy)]
pub struct TxdrR {
    bits: u32,
}

impl TxdrR {
    /// Bits 0:7 - 8-bit transmit data
    pub fn txdata(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct TxdrW {
    bits: u32,
}

impl TxdrW {
    /// Reset value
    pub fn reset_value() -> Self {
        TxdrW { bits: 0 }
    }
    /// Bits 0:7 - 8-bit transmit data
    pub fn txdata(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
