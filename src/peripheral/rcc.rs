#[repr(C)]
/// Reset and clock control
pub struct Rcc {
    /// 0x00 - Clock control register
    pub cr: Cr,
    /// 0x04 - Clock configuration register (RCC_CFGR)
    pub cfgr: Cfgr,
    /// 0x08 - Clock interrupt register (RCC_CIR)
    pub cir: Cir,
    /// 0x0c - APB2 peripheral reset register (RCC_APB2RSTR)
    pub apb2rstr: Apb2rstr,
    /// 0x10 - APB1 peripheral reset register (RCC_APB1RSTR)
    pub apb1rstr: Apb1rstr,
    /// 0x14 - AHB Peripheral Clock enable register (RCC_AHBENR)
    pub ahbenr: Ahbenr,
    /// 0x18 - APB2 peripheral clock enable register (RCC_APB2ENR)
    pub apb2enr: Apb2enr,
    /// 0x1c - APB1 peripheral clock enable register (RCC_APB1ENR)
    pub apb1enr: Apb1enr,
    /// 0x20 - Backup domain control register (RCC_BDCR)
    pub bdcr: Bdcr,
    /// 0x24 - Control/status register (RCC_CSR)
    pub csr: Csr,
    /// 0x28 - AHB peripheral reset register
    pub ahbrstr: Ahbrstr,
    /// 0x2c - Clock configuration register 2
    pub cfgr2: Cfgr2,
    /// 0x30 - Clock configuration register 3
    pub cfgr3: Cfgr3,
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
    /// Bit 0 - Internal High Speed clock enable
    pub fn hsion(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Internal High Speed clock ready flag
    pub fn hsirdy(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 3:7 - Internal High Speed clock trimming
    pub fn hsitrim(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:15 - Internal High Speed clock Calibration
    pub fn hsical(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 16 - External High Speed clock enable
    pub fn hseon(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - External High Speed clock ready flag
    pub fn hserdy(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - External High Speed clock Bypass
    pub fn hsebyp(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - Clock Security System enable
    pub fn csson(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 24 - PLL enable
    pub fn pllon(&self) -> bool {
        const OFFSET: u8 = 24;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 25 - PLL clock ready flag
    pub fn pllrdy(&self) -> bool {
        const OFFSET: u8 = 25;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct CrW {
    bits: u32,
}

impl CrW {
    /// Reset value
    pub fn reset_value() -> Self {
        CrW { bits: 131 }
    }
    /// Bit 0 - Internal High Speed clock enable
    pub fn hsion(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 3:7 - Internal High Speed clock trimming
    pub fn hsitrim(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 16 - External High Speed clock enable
    pub fn hseon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - External High Speed clock Bypass
    pub fn hsebyp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - Clock Security System enable
    pub fn csson(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 24 - PLL enable
    pub fn pllon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Cfgr {
    register: ::volatile_register::RW<u32>,
}

impl Cfgr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut CfgrW) -> &mut CfgrW
    {
        let mut rw = CfgrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> CfgrR {
        CfgrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: CfgrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct CfgrR {
    bits: u32,
}

impl CfgrR {
    /// Bits 0:1 - System clock Switch
    pub fn sw(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 2:3 - System Clock Switch Status
    pub fn sws(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 4:7 - AHB prescaler
    pub fn hpre(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:10 - APB Low speed prescaler (APB1)
    pub fn ppre1(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 11:13 - APB high speed prescaler (APB2)
    pub fn ppre2(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 11;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 16 - PLL entry clock source
    pub fn pllsrc(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - HSE divider for PLL entry
    pub fn pllxtpre(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 18:21 - PLL Multiplication Factor
    pub fn pllmul(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 18;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 22 - USB prescaler
    pub fn usbpres(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 24:26 - Microcontroller clock output
    pub fn mco(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 28 - Microcontroller Clock Output Flag
    pub fn mcof(&self) -> bool {
        const OFFSET: u8 = 28;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 23 - I2S external clock source selection
    pub fn i2ssrc(&self) -> bool {
        const OFFSET: u8 = 23;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct CfgrW {
    bits: u32,
}

impl CfgrW {
    /// Reset value
    pub fn reset_value() -> Self {
        CfgrW { bits: 0 }
    }
    /// Bits 0:1 - System clock Switch
    pub fn sw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 4:7 - AHB prescaler
    pub fn hpre(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:10 - APB Low speed prescaler (APB1)
    pub fn ppre1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 11:13 - APB high speed prescaler (APB2)
    pub fn ppre2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 16 - PLL entry clock source
    pub fn pllsrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - HSE divider for PLL entry
    pub fn pllxtpre(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 18:21 - PLL Multiplication Factor
    pub fn pllmul(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 22 - USB prescaler
    pub fn usbpres(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 24:26 - Microcontroller clock output
    pub fn mco(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 23 - I2S external clock source selection
    pub fn i2ssrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Cir {
    register: ::volatile_register::RW<u32>,
}

impl Cir {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut CirW) -> &mut CirW
    {
        let mut rw = CirW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> CirR {
        CirR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: CirW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct CirR {
    bits: u32,
}

impl CirR {
    /// Bit 0 - LSI Ready Interrupt flag
    pub fn lsirdyf(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - LSE Ready Interrupt flag
    pub fn lserdyf(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - HSI Ready Interrupt flag
    pub fn hsirdyf(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - HSE Ready Interrupt flag
    pub fn hserdyf(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - PLL Ready Interrupt flag
    pub fn pllrdyf(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Clock Security System Interrupt flag
    pub fn cssf(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - LSI Ready Interrupt Enable
    pub fn lsirdyie(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - LSE Ready Interrupt Enable
    pub fn lserdyie(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - HSI Ready Interrupt Enable
    pub fn hsirdyie(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - HSE Ready Interrupt Enable
    pub fn hserdyie(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - PLL Ready Interrupt Enable
    pub fn pllrdyie(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct CirW {
    bits: u32,
}

impl CirW {
    /// Reset value
    pub fn reset_value() -> Self {
        CirW { bits: 0 }
    }
    /// Bit 8 - LSI Ready Interrupt Enable
    pub fn lsirdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - LSE Ready Interrupt Enable
    pub fn lserdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - HSI Ready Interrupt Enable
    pub fn hsirdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - HSE Ready Interrupt Enable
    pub fn hserdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - PLL Ready Interrupt Enable
    pub fn pllrdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - LSI Ready Interrupt Clear
    pub fn lsirdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - LSE Ready Interrupt Clear
    pub fn lserdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - HSI Ready Interrupt Clear
    pub fn hsirdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - HSE Ready Interrupt Clear
    pub fn hserdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 20 - PLL Ready Interrupt Clear
    pub fn pllrdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 23 - Clock security system interrupt clear
    pub fn cssc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Apb2rstr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2rstr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2rstrW) -> &mut Apb2rstrW
    {
        let mut rw = Apb2rstrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Apb2rstrR {
        Apb2rstrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Apb2rstrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Apb2rstrR {
    bits: u32,
}

impl Apb2rstrR {
    /// Bit 0 - SYSCFG and COMP reset
    pub fn syscfgrst(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - TIM1 timer reset
    pub fn tim1rst(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - SPI 1 reset
    pub fn spi1rst(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - TIM8 timer reset
    pub fn tim8rst(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - USART1 reset
    pub fn usart1rst(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 16 - TIM15 timer reset
    pub fn tim15rst(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - TIM16 timer reset
    pub fn tim16rst(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - TIM17 timer reset
    pub fn tim17rst(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Apb2rstrW {
    bits: u32,
}

impl Apb2rstrW {
    /// Reset value
    pub fn reset_value() -> Self {
        Apb2rstrW { bits: 0 }
    }
    /// Bit 0 - SYSCFG and COMP reset
    pub fn syscfgrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - TIM1 timer reset
    pub fn tim1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - SPI 1 reset
    pub fn spi1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - TIM8 timer reset
    pub fn tim8rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - USART1 reset
    pub fn usart1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - TIM15 timer reset
    pub fn tim15rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - TIM16 timer reset
    pub fn tim16rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - TIM17 timer reset
    pub fn tim17rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Apb1rstr {
    register: ::volatile_register::RW<u32>,
}

impl Apb1rstr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1rstrW) -> &mut Apb1rstrW
    {
        let mut rw = Apb1rstrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Apb1rstrR {
        Apb1rstrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Apb1rstrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Apb1rstrR {
    bits: u32,
}

impl Apb1rstrR {
    /// Bit 0 - Timer 2 reset
    pub fn tim2rst(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Timer 3 reset
    pub fn tim3rst(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Timer 14 reset
    pub fn tim4rst(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Timer 6 reset
    pub fn tim6rst(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Timer 7 reset
    pub fn tim7rst(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Window watchdog reset
    pub fn wwdgrst(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - SPI2 reset
    pub fn spi2rst(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - SPI3 reset
    pub fn spi3rst(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - USART 2 reset
    pub fn usart2rst(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - USART3 reset
    pub fn usart3rst(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - UART 4 reset
    pub fn uart4rst(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 20 - UART 5 reset
    pub fn uart5rst(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - I2C1 reset
    pub fn i2c1rst(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 22 - I2C2 reset
    pub fn i2c2rst(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 23 - USB reset
    pub fn usbrst(&self) -> bool {
        const OFFSET: u8 = 23;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 25 - CAN reset
    pub fn canrst(&self) -> bool {
        const OFFSET: u8 = 25;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 28 - Power interface reset
    pub fn pwrrst(&self) -> bool {
        const OFFSET: u8 = 28;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 29 - DAC interface reset
    pub fn dacrst(&self) -> bool {
        const OFFSET: u8 = 29;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Apb1rstrW {
    bits: u32,
}

impl Apb1rstrW {
    /// Reset value
    pub fn reset_value() -> Self {
        Apb1rstrW { bits: 0 }
    }
    /// Bit 0 - Timer 2 reset
    pub fn tim2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Timer 3 reset
    pub fn tim3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Timer 14 reset
    pub fn tim4rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Timer 6 reset
    pub fn tim6rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Timer 7 reset
    pub fn tim7rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Window watchdog reset
    pub fn wwdgrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - SPI2 reset
    pub fn spi2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - SPI3 reset
    pub fn spi3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - USART 2 reset
    pub fn usart2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - USART3 reset
    pub fn usart3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - UART 4 reset
    pub fn uart4rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 20 - UART 5 reset
    pub fn uart5rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - I2C1 reset
    pub fn i2c1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 22 - I2C2 reset
    pub fn i2c2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 23 - USB reset
    pub fn usbrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 25 - CAN reset
    pub fn canrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 28 - Power interface reset
    pub fn pwrrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 29 - DAC interface reset
    pub fn dacrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Ahbenr {
    register: ::volatile_register::RW<u32>,
}

impl Ahbenr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut AhbenrW) -> &mut AhbenrW
    {
        let mut rw = AhbenrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> AhbenrR {
        AhbenrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: AhbenrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct AhbenrR {
    bits: u32,
}

impl AhbenrR {
    /// Bit 0 - DMA1 clock enable
    pub fn dmaen(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - DMA2 clock enable
    pub fn dma2en(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - SRAM interface clock enable
    pub fn sramen(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - FLITF clock enable
    pub fn flitfen(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - CRC clock enable
    pub fn crcen(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - I/O port A clock enable
    pub fn iopaen(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - I/O port B clock enable
    pub fn iopben(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - I/O port C clock enable
    pub fn iopcen(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 20 - I/O port D clock enable
    pub fn iopden(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - I/O port E clock enable
    pub fn iopeen(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 22 - I/O port F clock enable
    pub fn iopfen(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 24 - Touch sensing controller clock enable
    pub fn tscen(&self) -> bool {
        const OFFSET: u8 = 24;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 28 - ADC1 and ADC2 clock enable
    pub fn adc12en(&self) -> bool {
        const OFFSET: u8 = 28;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 29 - ADC3 and ADC4 clock enable
    pub fn adc34en(&self) -> bool {
        const OFFSET: u8 = 29;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct AhbenrW {
    bits: u32,
}

impl AhbenrW {
    /// Reset value
    pub fn reset_value() -> Self {
        AhbenrW { bits: 20 }
    }
    /// Bit 0 - DMA1 clock enable
    pub fn dmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - DMA2 clock enable
    pub fn dma2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - SRAM interface clock enable
    pub fn sramen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - FLITF clock enable
    pub fn flitfen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - CRC clock enable
    pub fn crcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - I/O port A clock enable
    pub fn iopaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - I/O port B clock enable
    pub fn iopben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - I/O port C clock enable
    pub fn iopcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 20 - I/O port D clock enable
    pub fn iopden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - I/O port E clock enable
    pub fn iopeen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 22 - I/O port F clock enable
    pub fn iopfen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 24 - Touch sensing controller clock enable
    pub fn tscen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 28 - ADC1 and ADC2 clock enable
    pub fn adc12en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 29 - ADC3 and ADC4 clock enable
    pub fn adc34en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Apb2enr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2enr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2enrW) -> &mut Apb2enrW
    {
        let mut rw = Apb2enrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Apb2enrR {
        Apb2enrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Apb2enrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Apb2enrR {
    bits: u32,
}

impl Apb2enrR {
    /// Bit 0 - SYSCFG clock enable
    pub fn syscfgen(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - TIM1 Timer clock enable
    pub fn tim1en(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - SPI 1 clock enable
    pub fn spi1en(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - TIM8 Timer clock enable
    pub fn tim8en(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - USART1 clock enable
    pub fn usart1en(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 16 - TIM15 timer clock enable
    pub fn tim15en(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - TIM16 timer clock enable
    pub fn tim16en(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - TIM17 timer clock enable
    pub fn tim17en(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Apb2enrW {
    bits: u32,
}

impl Apb2enrW {
    /// Reset value
    pub fn reset_value() -> Self {
        Apb2enrW { bits: 0 }
    }
    /// Bit 0 - SYSCFG clock enable
    pub fn syscfgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - TIM1 Timer clock enable
    pub fn tim1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - SPI 1 clock enable
    pub fn spi1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - TIM8 Timer clock enable
    pub fn tim8en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - USART1 clock enable
    pub fn usart1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - TIM15 timer clock enable
    pub fn tim15en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - TIM16 timer clock enable
    pub fn tim16en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - TIM17 timer clock enable
    pub fn tim17en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Apb1enr {
    register: ::volatile_register::RW<u32>,
}

impl Apb1enr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1enrW) -> &mut Apb1enrW
    {
        let mut rw = Apb1enrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Apb1enrR {
        Apb1enrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Apb1enrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Apb1enrR {
    bits: u32,
}

impl Apb1enrR {
    /// Bit 0 - Timer 2 clock enable
    pub fn tim2en(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Timer 3 clock enable
    pub fn tim3en(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Timer 4 clock enable
    pub fn tim4en(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Timer 6 clock enable
    pub fn tim6en(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Timer 7 clock enable
    pub fn tim7en(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Window watchdog clock enable
    pub fn wwdgen(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - SPI 2 clock enable
    pub fn spi2en(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - SPI 3 clock enable
    pub fn spi3en(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - USART 2 clock enable
    pub fn usart2en(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - I2C 1 clock enable
    pub fn i2c1en(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 22 - I2C 2 clock enable
    pub fn i2c2en(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 23 - USB clock enable
    pub fn usben(&self) -> bool {
        const OFFSET: u8 = 23;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 25 - CAN clock enable
    pub fn canen(&self) -> bool {
        const OFFSET: u8 = 25;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 28 - Power interface clock enable
    pub fn pwren(&self) -> bool {
        const OFFSET: u8 = 28;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 29 - DAC interface clock enable
    pub fn dacen(&self) -> bool {
        const OFFSET: u8 = 29;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Apb1enrW {
    bits: u32,
}

impl Apb1enrW {
    /// Reset value
    pub fn reset_value() -> Self {
        Apb1enrW { bits: 0 }
    }
    /// Bit 0 - Timer 2 clock enable
    pub fn tim2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Timer 3 clock enable
    pub fn tim3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Timer 4 clock enable
    pub fn tim4en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Timer 6 clock enable
    pub fn tim6en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Timer 7 clock enable
    pub fn tim7en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Window watchdog clock enable
    pub fn wwdgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - SPI 2 clock enable
    pub fn spi2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - SPI 3 clock enable
    pub fn spi3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - USART 2 clock enable
    pub fn usart2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - I2C 1 clock enable
    pub fn i2c1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 22 - I2C 2 clock enable
    pub fn i2c2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 23 - USB clock enable
    pub fn usben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 25 - CAN clock enable
    pub fn canen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 28 - Power interface clock enable
    pub fn pwren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 29 - DAC interface clock enable
    pub fn dacen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Bdcr {
    register: ::volatile_register::RW<u32>,
}

impl Bdcr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut BdcrW) -> &mut BdcrW
    {
        let mut rw = BdcrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> BdcrR {
        BdcrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: BdcrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct BdcrR {
    bits: u32,
}

impl BdcrR {
    /// Bit 0 - External Low Speed oscillator enable
    pub fn lseon(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - External Low Speed oscillator ready
    pub fn lserdy(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - External Low Speed oscillator bypass
    pub fn lsebyp(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 3:4 - LSE oscillator drive capability
    pub fn lsedrv(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 3;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 8:9 - RTC clock source selection
    pub fn rtcsel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 15 - RTC clock enable
    pub fn rtcen(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 16 - Backup domain software reset
    pub fn bdrst(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct BdcrW {
    bits: u32,
}

impl BdcrW {
    /// Reset value
    pub fn reset_value() -> Self {
        BdcrW { bits: 0 }
    }
    /// Bit 0 - External Low Speed oscillator enable
    pub fn lseon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - External Low Speed oscillator bypass
    pub fn lsebyp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 3:4 - LSE oscillator drive capability
    pub fn lsedrv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 8:9 - RTC clock source selection
    pub fn rtcsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 15 - RTC clock enable
    pub fn rtcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - Backup domain software reset
    pub fn bdrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Csr {
    register: ::volatile_register::RW<u32>,
}

impl Csr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut CsrW) -> &mut CsrW
    {
        let mut rw = CsrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> CsrR {
        CsrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: CsrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct CsrR {
    bits: u32,
}

impl CsrR {
    /// Bit 0 - Internal low speed oscillator enable
    pub fn lsion(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Internal low speed oscillator ready
    pub fn lsirdy(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 24 - Remove reset flag
    pub fn rmvf(&self) -> bool {
        const OFFSET: u8 = 24;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 25 - Option byte loader reset flag
    pub fn oblrstf(&self) -> bool {
        const OFFSET: u8 = 25;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 26 - PIN reset flag
    pub fn pinrstf(&self) -> bool {
        const OFFSET: u8 = 26;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 27 - POR/PDR reset flag
    pub fn porrstf(&self) -> bool {
        const OFFSET: u8 = 27;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 28 - Software reset flag
    pub fn sftrstf(&self) -> bool {
        const OFFSET: u8 = 28;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 29 - Independent watchdog reset flag
    pub fn iwdgrstf(&self) -> bool {
        const OFFSET: u8 = 29;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 30 - Window watchdog reset flag
    pub fn wwdgrstf(&self) -> bool {
        const OFFSET: u8 = 30;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 31 - Low-power reset flag
    pub fn lpwrrstf(&self) -> bool {
        const OFFSET: u8 = 31;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct CsrW {
    bits: u32,
}

impl CsrW {
    /// Reset value
    pub fn reset_value() -> Self {
        CsrW { bits: 201326592 }
    }
    /// Bit 0 - Internal low speed oscillator enable
    pub fn lsion(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 24 - Remove reset flag
    pub fn rmvf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 25 - Option byte loader reset flag
    pub fn oblrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 26 - PIN reset flag
    pub fn pinrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 27 - POR/PDR reset flag
    pub fn porrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 28 - Software reset flag
    pub fn sftrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 29 - Independent watchdog reset flag
    pub fn iwdgrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 30 - Window watchdog reset flag
    pub fn wwdgrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 31 - Low-power reset flag
    pub fn lpwrrstf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Ahbrstr {
    register: ::volatile_register::RW<u32>,
}

impl Ahbrstr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut AhbrstrW) -> &mut AhbrstrW
    {
        let mut rw = AhbrstrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> AhbrstrR {
        AhbrstrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: AhbrstrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct AhbrstrR {
    bits: u32,
}

impl AhbrstrR {
    /// Bit 17 - I/O port A reset
    pub fn ioparst(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - I/O port B reset
    pub fn iopbrst(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - I/O port C reset
    pub fn iopcrst(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 20 - I/O port D reset
    pub fn iopdrst(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - I/O port E reset
    pub fn ioperst(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 22 - I/O port F reset
    pub fn iopfrst(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 24 - Touch sensing controller reset
    pub fn tscrst(&self) -> bool {
        const OFFSET: u8 = 24;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 28 - ADC1 and ADC2 reset
    pub fn adc12rst(&self) -> bool {
        const OFFSET: u8 = 28;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 29 - ADC3 and ADC4 reset
    pub fn adc34rst(&self) -> bool {
        const OFFSET: u8 = 29;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct AhbrstrW {
    bits: u32,
}

impl AhbrstrW {
    /// Reset value
    pub fn reset_value() -> Self {
        AhbrstrW { bits: 0 }
    }
    /// Bit 17 - I/O port A reset
    pub fn ioparst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - I/O port B reset
    pub fn iopbrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - I/O port C reset
    pub fn iopcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 20 - I/O port D reset
    pub fn iopdrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - I/O port E reset
    pub fn ioperst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 22 - I/O port F reset
    pub fn iopfrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 24 - Touch sensing controller reset
    pub fn tscrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 28 - ADC1 and ADC2 reset
    pub fn adc12rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 29 - ADC3 and ADC4 reset
    pub fn adc34rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Cfgr2 {
    register: ::volatile_register::RW<u32>,
}

impl Cfgr2 {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Cfgr2W) -> &mut Cfgr2W
    {
        let mut rw = Cfgr2W { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Cfgr2R {
        Cfgr2R { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Cfgr2W) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Cfgr2R {
    bits: u32,
}

impl Cfgr2R {
    /// Bits 0:3 - PREDIV division factor
    pub fn prediv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 4:8 - ADC1 and ADC2 prescaler
    pub fn adc12pres(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 9:13 - ADC3 and ADC4 prescaler
    pub fn adc34pres(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 9;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct Cfgr2W {
    bits: u32,
}

impl Cfgr2W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cfgr2W { bits: 0 }
    }
    /// Bits 0:3 - PREDIV division factor
    pub fn prediv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 4:8 - ADC1 and ADC2 prescaler
    pub fn adc12pres(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 9:13 - ADC3 and ADC4 prescaler
    pub fn adc34pres(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 9;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Cfgr3 {
    register: ::volatile_register::RW<u32>,
}

impl Cfgr3 {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Cfgr3W) -> &mut Cfgr3W
    {
        let mut rw = Cfgr3W { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Cfgr3R {
        Cfgr3R { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Cfgr3W) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Cfgr3R {
    bits: u32,
}

impl Cfgr3R {
    /// Bits 0:1 - USART1 clock source selection
    pub fn usart1sw(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 4 - I2C1 clock source selection
    pub fn i2c1sw(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - I2C2 clock source selection
    pub fn i2c2sw(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 16:17 - USART2 clock source selection
    pub fn usart2sw(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 18:19 - USART3 clock source selection
    pub fn usart3sw(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 8 - Timer1 clock source selection
    pub fn tim1sw(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Timer8 clock source selection
    pub fn tim8sw(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 20:21 - UART4 clock source selection
    pub fn uart4sw(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 22:23 - UART5 clock source selection
    pub fn uart5sw(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct Cfgr3W {
    bits: u32,
}

impl Cfgr3W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cfgr3W { bits: 0 }
    }
    /// Bits 0:1 - USART1 clock source selection
    pub fn usart1sw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 4 - I2C1 clock source selection
    pub fn i2c1sw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - I2C2 clock source selection
    pub fn i2c2sw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 16:17 - USART2 clock source selection
    pub fn usart2sw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 18:19 - USART3 clock source selection
    pub fn usart3sw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 8 - Timer1 clock source selection
    pub fn tim1sw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Timer8 clock source selection
    pub fn tim8sw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 20:21 - UART4 clock source selection
    pub fn uart4sw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 22:23 - UART5 clock source selection
    pub fn uart5sw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
