# [ doc = "Debug support" ]
# [ repr ( C ) ]
pub struct Dbgmcu {
    # [ doc = "0x00 - MCU Device ID Code Register" ]
    pub idcode: Idcode,
    # [ doc = "0x04 - Debug MCU Configuration Register" ]
    pub cr: Cr,
    # [ doc = "0x08 - APB Low Freeze Register" ]
    pub apb1fz: Apb1fz,
    # [ doc = "0x0c - APB High Freeze Register" ]
    pub apb2fz: Apb2fz,
}

# [ repr ( C ) ]
pub struct Idcode {
    register: ::volatile_register::RO<u32>,
}

impl Idcode {
    pub fn read(&self) -> IdcodeR {
        IdcodeR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdcodeR {
    bits: u32,
}

impl IdcodeR {
    # [ doc = "Bits 0:11 - Device Identifier" ]
    pub fn dev_id(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Revision Identifier" ]
    pub fn rev_id(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdcodeW {
    bits: u32,
}

impl IdcodeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdcodeW { bits: 0u32 }
    }
    # [ doc = "Bits 0:11 - Device Identifier" ]
    pub fn dev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Revision Identifier" ]
    pub fn rev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

impl Cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CrR, &'w mut CrW) -> &'w mut CrW
    {
        let bits = self.register.read();
        let r = CrR { bits: bits };
        let mut w = CrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CrR {
        CrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CrW) -> &mut CrW
    {
        let mut w = CrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrR {
    bits: u32,
}

impl CrR {
    # [ doc = "Bit 0 - Debug Sleep mode" ]
    pub fn dbg_sleep(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Debug Stop Mode" ]
    pub fn dbg_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Debug Standby Mode" ]
    pub fn dbg_standby(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Trace pin assignment control" ]
    pub fn trace_ioen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Trace pin assignment control" ]
    pub fn trace_mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrW {
    bits: u32,
}

impl CrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Debug Sleep mode" ]
    pub fn dbg_sleep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Debug Stop Mode" ]
    pub fn dbg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Debug Standby Mode" ]
    pub fn dbg_standby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Trace pin assignment control" ]
    pub fn trace_ioen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Trace pin assignment control" ]
    pub fn trace_mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Apb1fz {
    register: ::volatile_register::RW<u32>,
}

impl Apb1fz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1fzR, &'w mut Apb1fzW) -> &'w mut Apb1fzW
    {
        let bits = self.register.read();
        let r = Apb1fzR { bits: bits };
        let mut w = Apb1fzW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1fzR {
        Apb1fzR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1fzW) -> &mut Apb1fzW
    {
        let mut w = Apb1fzW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1fzR {
    bits: u32,
}

impl Apb1fzR {
    # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
    pub fn dbg_tim2_stop(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Debug Timer 3 stopped when Core is halted" ]
    pub fn dbg_tim3_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Debug Timer 4 stopped when Core is halted" ]
    pub fn dbg_tim4_stop(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Debug Timer 5 stopped when Core is halted" ]
    pub fn dbg_tim5_stop(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
    pub fn dbg_tim6_stop(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Debug Timer 7 stopped when Core is halted" ]
    pub fn dbg_tim7_stop(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Debug Timer 12 stopped when Core is halted" ]
    pub fn dbg_tim12_stop(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Debug Timer 13 stopped when Core is halted" ]
    pub fn dbg_tim13_stop(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Debug Timer 14 stopped when Core is halted" ]
    pub fn dbg_timer14_stop(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Debug Timer 18 stopped when Core is halted" ]
    pub fn dbg_tim18_stop(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
    pub fn dbg_wwdg_stop(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
    pub fn dbg_iwdg_stop(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted" ]
    pub fn i2c1_smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted" ]
    pub fn i2c2_smbus_timeout(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Debug CAN stopped when core is halted" ]
    pub fn dbg_can_stop(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1fzW {
    bits: u32,
}

impl Apb1fzW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1fzW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
    pub fn dbg_tim2_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Debug Timer 3 stopped when Core is halted" ]
    pub fn dbg_tim3_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Debug Timer 4 stopped when Core is halted" ]
    pub fn dbg_tim4_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Debug Timer 5 stopped when Core is halted" ]
    pub fn dbg_tim5_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
    pub fn dbg_tim6_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Debug Timer 7 stopped when Core is halted" ]
    pub fn dbg_tim7_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Debug Timer 12 stopped when Core is halted" ]
    pub fn dbg_tim12_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Debug Timer 13 stopped when Core is halted" ]
    pub fn dbg_tim13_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Debug Timer 14 stopped when Core is halted" ]
    pub fn dbg_timer14_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Debug Timer 18 stopped when Core is halted" ]
    pub fn dbg_tim18_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
    pub fn dbg_wwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
    pub fn dbg_iwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - SMBUS timeout mode stopped when Core is halted" ]
    pub fn i2c1_smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - SMBUS timeout mode stopped when Core is halted" ]
    pub fn i2c2_smbus_timeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Debug CAN stopped when core is halted" ]
    pub fn dbg_can_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Apb2fz {
    register: ::volatile_register::RW<u32>,
}

impl Apb2fz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2fzR, &'w mut Apb2fzW) -> &'w mut Apb2fzW
    {
        let bits = self.register.read();
        let r = Apb2fzR { bits: bits };
        let mut w = Apb2fzW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2fzR {
        Apb2fzR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2fzW) -> &mut Apb2fzW
    {
        let mut w = Apb2fzW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2fzR {
    bits: u32,
}

impl Apb2fzR {
    # [ doc = "Bit 2 - Debug Timer 15 stopped when Core is halted" ]
    pub fn dbg_tim15_stop(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Debug Timer 16 stopped when Core is halted" ]
    pub fn dbg_tim16_stop(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Debug Timer 17 stopped when Core is halted" ]
    pub fn dbg_tim17_sto(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Debug Timer 19 stopped when Core is halted" ]
    pub fn dbg_tim19_stop(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2fzW {
    bits: u32,
}

impl Apb2fzW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2fzW { bits: 0u32 }
    }
    # [ doc = "Bit 2 - Debug Timer 15 stopped when Core is halted" ]
    pub fn dbg_tim15_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Debug Timer 16 stopped when Core is halted" ]
    pub fn dbg_tim16_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Debug Timer 17 stopped when Core is halted" ]
    pub fn dbg_tim17_sto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Debug Timer 19 stopped when Core is halted" ]
    pub fn dbg_tim19_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
