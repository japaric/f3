#[repr(C)]
/// Debug support
pub struct Dbgmcu {
    /// 0x00 - MCU Device ID Code Register
    pub idcode: Idcode,
    /// 0x04 - Debug MCU Configuration Register
    pub cr: Cr,
    /// 0x08 - APB Low Freeze Register
    pub apb1fz: Apb1fz,
    /// 0x0c - APB High Freeze Register
    pub apb2fz: Apb2fz,
}

pub struct Idcode {
    register: ::volatile_register::RO<u32>,
}

impl Idcode {
    pub fn read(&self) -> IdcodeR {
        IdcodeR { bits: self.register.read() }
    }
}

#[derive(Clone, Copy)]
pub struct IdcodeR {
    bits: u32,
}

impl IdcodeR {
    /// Bits 0:11 - Device Identifier
    pub fn dev_id(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    /// Bits 16:31 - Revision Identifier
    pub fn rev_id(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

#[derive(Clone, Copy)]
pub struct IdcodeW {
    bits: u32,
}

impl IdcodeW {
    /// Reset value
    pub fn reset_value() -> Self {
        IdcodeW { bits: 0 }
    }
    /// Bits 0:11 - Device Identifier
    pub fn dev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:31 - Revision Identifier
    pub fn rev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

impl Cr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut CrW) -> &mut CrW
    {
        let mut rw = CrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> CrR {
        CrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: CrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct CrR {
    bits: u32,
}

impl CrR {
    /// Bit 0 - Debug Sleep mode
    pub fn dbg_sleep(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Debug Stop Mode
    pub fn dbg_stop(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Debug Standby Mode
    pub fn dbg_standby(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Trace pin assignment control
    pub fn trace_ioen(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 6:7 - Trace pin assignment control
    pub fn trace_mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct CrW {
    bits: u32,
}

impl CrW {
    /// Reset value
    pub fn reset_value() -> Self {
        CrW { bits: 0 }
    }
    /// Bit 0 - Debug Sleep mode
    pub fn dbg_sleep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Debug Stop Mode
    pub fn dbg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Debug Standby Mode
    pub fn dbg_standby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Trace pin assignment control
    pub fn trace_ioen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 6:7 - Trace pin assignment control
    pub fn trace_mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Apb1fz {
    register: ::volatile_register::RW<u32>,
}

impl Apb1fz {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1fzW) -> &mut Apb1fzW
    {
        let mut rw = Apb1fzW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Apb1fzR {
        Apb1fzR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Apb1fzW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Apb1fzR {
    bits: u32,
}

impl Apb1fzR {
    /// Bit 0 - Debug Timer 2 stopped when Core is halted
    pub fn dbg_tim2_stop(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Debug Timer 3 stopped when Core is halted
    pub fn dbg_tim3_stop(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Debug Timer 4 stopped when Core is halted
    pub fn dbg_tim4_stop(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Debug Timer 5 stopped when Core is halted
    pub fn dbg_tim5_stop(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Debug Timer 6 stopped when Core is halted
    pub fn dbg_tim6_stop(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Debug Timer 7 stopped when Core is halted
    pub fn dbg_tim7_stop(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Debug Timer 12 stopped when Core is halted
    pub fn dbg_tim12_stop(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Debug Timer 13 stopped when Core is halted
    pub fn dbg_tim13_stop(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Debug Timer 14 stopped when Core is halted
    pub fn dbg_timer14_stop(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Debug Timer 18 stopped when Core is halted
    pub fn dbg_tim18_stop(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Debug RTC stopped when Core is halted
    pub fn dbg_rtc_stop(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Debug Window Wachdog stopped when Core is halted
    pub fn dbg_wwdg_stop(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Debug Independent Wachdog stopped when Core is halted
    pub fn dbg_iwdg_stop(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - SMBUS timeout mode stopped when Core is halted
    pub fn i2c1_smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 22 - SMBUS timeout mode stopped when Core is halted
    pub fn i2c2_smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 25 - Debug CAN stopped when core is halted
    pub fn dbg_can_stop(&self) -> bool {
        const OFFSET: u8 = 25;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Apb1fzW {
    bits: u32,
}

impl Apb1fzW {
    /// Reset value
    pub fn reset_value() -> Self {
        Apb1fzW { bits: 0 }
    }
    /// Bit 0 - Debug Timer 2 stopped when Core is halted
    pub fn dbg_tim2_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Debug Timer 3 stopped when Core is halted
    pub fn dbg_tim3_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Debug Timer 4 stopped when Core is halted
    pub fn dbg_tim4_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Debug Timer 5 stopped when Core is halted
    pub fn dbg_tim5_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Debug Timer 6 stopped when Core is halted
    pub fn dbg_tim6_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Debug Timer 7 stopped when Core is halted
    pub fn dbg_tim7_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Debug Timer 12 stopped when Core is halted
    pub fn dbg_tim12_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Debug Timer 13 stopped when Core is halted
    pub fn dbg_tim13_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Debug Timer 14 stopped when Core is halted
    pub fn dbg_timer14_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Debug Timer 18 stopped when Core is halted
    pub fn dbg_tim18_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Debug RTC stopped when Core is halted
    pub fn dbg_rtc_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Debug Window Wachdog stopped when Core is halted
    pub fn dbg_wwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Debug Independent Wachdog stopped when Core is halted
    pub fn dbg_iwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - SMBUS timeout mode stopped when Core is halted
    pub fn i2c1_smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 22 - SMBUS timeout mode stopped when Core is halted
    pub fn i2c2_smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 25 - Debug CAN stopped when core is halted
    pub fn dbg_can_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Apb2fz {
    register: ::volatile_register::RW<u32>,
}

impl Apb2fz {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2fzW) -> &mut Apb2fzW
    {
        let mut rw = Apb2fzW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Apb2fzR {
        Apb2fzR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Apb2fzW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Apb2fzR {
    bits: u32,
}

impl Apb2fzR {
    /// Bit 2 - Debug Timer 15 stopped when Core is halted
    pub fn dbg_tim15_stop(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Debug Timer 16 stopped when Core is halted
    pub fn dbg_tim16_stop(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Debug Timer 17 stopped when Core is halted
    pub fn dbg_tim17_sto(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Debug Timer 19 stopped when Core is halted
    pub fn dbg_tim19_stop(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Apb2fzW {
    bits: u32,
}

impl Apb2fzW {
    /// Reset value
    pub fn reset_value() -> Self {
        Apb2fzW { bits: 0 }
    }
    /// Bit 2 - Debug Timer 15 stopped when Core is halted
    pub fn dbg_tim15_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Debug Timer 16 stopped when Core is halted
    pub fn dbg_tim16_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Debug Timer 17 stopped when Core is halted
    pub fn dbg_tim17_sto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Debug Timer 19 stopped when Core is halted
    pub fn dbg_tim19_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
