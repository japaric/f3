#[repr(C)]
/// Universal synchronous asynchronous receiver transmitter
pub struct Usart {
    /// 0x00 - Control register 1
    pub cr1: Cr1,
    /// 0x04 - Control register 2
    pub cr2: Cr2,
    /// 0x08 - Control register 3
    pub cr3: Cr3,
    /// 0x0c - Baud rate register
    pub brr: Brr,
    /// 0x10 - Guard time and prescaler register
    pub gtpr: Gtpr,
    /// 0x14 - Receiver timeout register
    pub rtor: Rtor,
    /// 0x18 - Request register
    pub rqr: Rqr,
    /// 0x1c - Interrupt & status register
    pub isr: Isr,
    /// 0x20 - Interrupt flag clear register
    pub icr: Icr,
    /// 0x24 - Receive data register
    pub rdr: Rdr,
    /// 0x28 - Transmit data register
    pub tdr: Tdr,
}

pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr1W) -> &mut Cr1W
    {
        let mut rw = Cr1W { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Cr1R {
        Cr1R { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Cr1W) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Cr1R {
    bits: u32,
}

impl Cr1R {
    /// Bit 27 - End of Block interrupt enable
    pub fn eobie(&self) -> bool {
        const OFFSET: u8 = 27;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 26 - Receiver timeout interrupt enable
    pub fn rtoie(&self) -> bool {
        const OFFSET: u8 = 26;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 21:25 - Driver Enable assertion time
    pub fn deat(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 21;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 16:20 - Driver Enable deassertion time
    pub fn dedt(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 15 - Oversampling mode
    pub fn over8(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Character match interrupt enable
    pub fn cmie(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - Mute mode enable
    pub fn mme(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Word length
    pub fn m(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Receiver wakeup method
    pub fn wake(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Parity control enable
    pub fn pce(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Parity selection
    pub fn ps(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - PE interrupt enable
    pub fn peie(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - interrupt enable
    pub fn txeie(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Transmission complete interrupt enable
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - RXNE interrupt enable
    pub fn rxneie(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - IDLE interrupt enable
    pub fn idleie(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Transmitter enable
    pub fn te(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Receiver enable
    pub fn re(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - USART enable in Stop mode
    pub fn uesm(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - USART enable
    pub fn ue(&self) -> bool {
        const OFFSET: u8 = 0;
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
    /// Bit 27 - End of Block interrupt enable
    pub fn eobie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 26 - Receiver timeout interrupt enable
    pub fn rtoie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 21:25 - Driver Enable assertion time
    pub fn deat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 16:20 - Driver Enable deassertion time
    pub fn dedt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 15 - Oversampling mode
    pub fn over8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Character match interrupt enable
    pub fn cmie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - Mute mode enable
    pub fn mme(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Word length
    pub fn m(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Receiver wakeup method
    pub fn wake(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Parity control enable
    pub fn pce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Parity selection
    pub fn ps(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - PE interrupt enable
    pub fn peie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - interrupt enable
    pub fn txeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Transmission complete interrupt enable
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - RXNE interrupt enable
    pub fn rxneie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - IDLE interrupt enable
    pub fn idleie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Transmitter enable
    pub fn te(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Receiver enable
    pub fn re(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - USART enable in Stop mode
    pub fn uesm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - USART enable
    pub fn ue(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
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
        where F: FnOnce(&mut Cr2W) -> &mut Cr2W
    {
        let mut rw = Cr2W { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Cr2R {
        Cr2R { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Cr2W) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Cr2R {
    bits: u32,
}

impl Cr2R {
    /// Bits 28:31 - Address of the USART node
    pub fn add4(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 28;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 24:27 - Address of the USART node
    pub fn add0(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 23 - Receiver timeout enable
    pub fn rtoen(&self) -> bool {
        const OFFSET: u8 = 23;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 21:22 - Auto baud rate mode
    pub fn abrmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 20 - Auto baud rate enable
    pub fn abren(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - Most significant bit first
    pub fn msbfirst(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - Binary data inversion
    pub fn datainv(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - TX pin active level inversion
    pub fn txinv(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 16 - RX pin active level inversion
    pub fn rxinv(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Swap TX/RX pins
    pub fn swap(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - LIN mode enable
    pub fn linen(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 12:13 - STOP bits
    pub fn stop(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 11 - Clock enable
    pub fn clken(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - Clock polarity
    pub fn cpol(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - Clock phase
    pub fn cpha(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - Last bit clock pulse
    pub fn lbcl(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - LIN break detection interrupt enable
    pub fn lbdie(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - LIN break detection length
    pub fn lbdl(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - 7-bit Address Detection/4-bit Address Detection
    pub fn addm7(&self) -> bool {
        const OFFSET: u8 = 4;
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
    /// Bits 28:31 - Address of the USART node
    pub fn add4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 24:27 - Address of the USART node
    pub fn add0(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 23 - Receiver timeout enable
    pub fn rtoen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 21:22 - Auto baud rate mode
    pub fn abrmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 20 - Auto baud rate enable
    pub fn abren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - Most significant bit first
    pub fn msbfirst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - Binary data inversion
    pub fn datainv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - TX pin active level inversion
    pub fn txinv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - RX pin active level inversion
    pub fn rxinv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - Swap TX/RX pins
    pub fn swap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - LIN mode enable
    pub fn linen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 12:13 - STOP bits
    pub fn stop(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 11 - Clock enable
    pub fn clken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - Clock polarity
    pub fn cpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - Clock phase
    pub fn cpha(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - Last bit clock pulse
    pub fn lbcl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - LIN break detection interrupt enable
    pub fn lbdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - LIN break detection length
    pub fn lbdl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - 7-bit Address Detection/4-bit Address Detection
    pub fn addm7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Cr3 {
    register: ::volatile_register::RW<u32>,
}

impl Cr3 {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr3W) -> &mut Cr3W
    {
        let mut rw = Cr3W { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> Cr3R {
        Cr3R { bits: self.register.read() }
    }
    pub fn write(&mut self, value: Cr3W) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct Cr3R {
    bits: u32,
}

impl Cr3R {
    /// Bit 22 - Wakeup from Stop mode interrupt enable
    pub fn wufie(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    pub fn wus(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 17:19 - Smartcard auto-retry count
    pub fn scarcnt(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 17;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bit 15 - Driver enable polarity selection
    pub fn dep(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Driver enable mode
    pub fn dem(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 13 - DMA Disable on Reception Error
    pub fn ddre(&self) -> bool {
        const OFFSET: u8 = 13;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - Overrun Disable
    pub fn ovrdis(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - One sample bit method enable
    pub fn onebit(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - CTS interrupt enable
    pub fn ctsie(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - CTS enable
    pub fn ctse(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - RTS enable
    pub fn rtse(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - DMA enable transmitter
    pub fn dmat(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - DMA enable receiver
    pub fn dmar(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Smartcard mode enable
    pub fn scen(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Smartcard NACK enable
    pub fn nack(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Half-duplex selection
    pub fn hdsel(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - IrDA low-power
    pub fn irlp(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - IrDA mode enable
    pub fn iren(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Error interrupt enable
    pub fn eie(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct Cr3W {
    bits: u32,
}

impl Cr3W {
    /// Reset value
    pub fn reset_value() -> Self {
        Cr3W { bits: 0 }
    }
    /// Bit 22 - Wakeup from Stop mode interrupt enable
    pub fn wufie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    pub fn wus(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 17:19 - Smartcard auto-retry count
    pub fn scarcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bit 15 - Driver enable polarity selection
    pub fn dep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Driver enable mode
    pub fn dem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 13 - DMA Disable on Reception Error
    pub fn ddre(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - Overrun Disable
    pub fn ovrdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - One sample bit method enable
    pub fn onebit(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - CTS interrupt enable
    pub fn ctsie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - CTS enable
    pub fn ctse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - RTS enable
    pub fn rtse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - DMA enable transmitter
    pub fn dmat(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - DMA enable receiver
    pub fn dmar(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Smartcard mode enable
    pub fn scen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Smartcard NACK enable
    pub fn nack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Half-duplex selection
    pub fn hdsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - IrDA low-power
    pub fn irlp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - IrDA mode enable
    pub fn iren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Error interrupt enable
    pub fn eie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Brr {
    register: ::volatile_register::RW<u32>,
}

impl Brr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut BrrW) -> &mut BrrW
    {
        let mut rw = BrrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> BrrR {
        BrrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: BrrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct BrrR {
    bits: u32,
}

impl BrrR {
    /// Bits 4:15 - mantissa of USARTDIV
    pub fn div_mantissa(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 4;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    /// Bits 0:3 - fraction of USARTDIV
    pub fn div_fraction(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct BrrW {
    bits: u32,
}

impl BrrW {
    /// Reset value
    pub fn reset_value() -> Self {
        BrrW { bits: 0 }
    }
    /// Bits 4:15 - mantissa of USARTDIV
    pub fn div_mantissa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 4;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:3 - fraction of USARTDIV
    pub fn div_fraction(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Gtpr {
    register: ::volatile_register::RW<u32>,
}

impl Gtpr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut GtprW) -> &mut GtprW
    {
        let mut rw = GtprW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> GtprR {
        GtprR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: GtprW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct GtprR {
    bits: u32,
}

impl GtprR {
    /// Bits 8:15 - Guard time value
    pub fn gt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 0:7 - Prescaler value
    pub fn psc(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

#[derive(Clone, Copy)]
pub struct GtprW {
    bits: u32,
}

impl GtprW {
    /// Reset value
    pub fn reset_value() -> Self {
        GtprW { bits: 0 }
    }
    /// Bits 8:15 - Guard time value
    pub fn gt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:7 - Prescaler value
    pub fn psc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Rtor {
    register: ::volatile_register::RW<u32>,
}

impl Rtor {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut RtorW) -> &mut RtorW
    {
        let mut rw = RtorW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> RtorR {
        RtorR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: RtorW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct RtorR {
    bits: u32,
}

impl RtorR {
    /// Bits 24:31 - Block Length
    pub fn blen(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    /// Bits 0:23 - Receiver timeout value
    pub fn rto(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

#[derive(Clone, Copy)]
pub struct RtorW {
    bits: u32,
}

impl RtorW {
    /// Reset value
    pub fn reset_value() -> Self {
        RtorW { bits: 0 }
    }
    /// Bits 24:31 - Block Length
    pub fn blen(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    /// Bits 0:23 - Receiver timeout value
    pub fn rto(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Rqr {
    register: ::volatile_register::RW<u32>,
}

impl Rqr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut RqrW) -> &mut RqrW
    {
        let mut rw = RqrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> RqrR {
        RqrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: RqrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct RqrR {
    bits: u32,
}

impl RqrR {
    /// Bit 4 - Transmit data flush request
    pub fn txfrq(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Receive data flush request
    pub fn rxfrq(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Mute mode request
    pub fn mmrq(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Send break request
    pub fn sbkrq(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Auto baud rate request
    pub fn abrrq(&self) -> bool {
        const OFFSET: u8 = 0;
        self.bits & (1 << OFFSET) != 0
    }
}

#[derive(Clone, Copy)]
pub struct RqrW {
    bits: u32,
}

impl RqrW {
    /// Reset value
    pub fn reset_value() -> Self {
        RqrW { bits: 0 }
    }
    /// Bit 4 - Transmit data flush request
    pub fn txfrq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Receive data flush request
    pub fn rxfrq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Mute mode request
    pub fn mmrq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Send break request
    pub fn sbkrq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Auto baud rate request
    pub fn abrrq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Isr {
    register: ::volatile_register::RO<u32>,
}

impl Isr {
    pub fn read(&self) -> IsrR {
        IsrR { bits: self.register.read() }
    }
}

#[derive(Clone, Copy)]
pub struct IsrR {
    bits: u32,
}

impl IsrR {
    /// Bit 22 - Receive enable acknowledge flag
    pub fn reack(&self) -> bool {
        const OFFSET: u8 = 22;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 21 - Transmit enable acknowledge flag
    pub fn teack(&self) -> bool {
        const OFFSET: u8 = 21;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 20 - Wakeup from Stop mode flag
    pub fn wuf(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 19 - Receiver wakeup from Mute mode
    pub fn rwu(&self) -> bool {
        const OFFSET: u8 = 19;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 18 - Send break flag
    pub fn sbkf(&self) -> bool {
        const OFFSET: u8 = 18;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - character match flag
    pub fn cmf(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 16 - Busy flag
    pub fn busy(&self) -> bool {
        const OFFSET: u8 = 16;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 15 - Auto baud rate flag
    pub fn abrf(&self) -> bool {
        const OFFSET: u8 = 15;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 14 - Auto baud rate error
    pub fn abre(&self) -> bool {
        const OFFSET: u8 = 14;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - End of block flag
    pub fn eobf(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Receiver timeout
    pub fn rtof(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 10 - CTS flag
    pub fn cts(&self) -> bool {
        const OFFSET: u8 = 10;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - CTS interrupt flag
    pub fn ctsif(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - LIN break detection flag
    pub fn lbdf(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 7 - Transmit data register empty
    pub fn txe(&self) -> bool {
        const OFFSET: u8 = 7;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Transmission complete
    pub fn tc(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 5 - Read data register not empty
    pub fn rxne(&self) -> bool {
        const OFFSET: u8 = 5;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Idle line detected
    pub fn idle(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Overrun error
    pub fn ore(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Noise detected flag
    pub fn nf(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Framing error
    pub fn fe(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Parity error
    pub fn pe(&self) -> bool {
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
        IsrW { bits: 192 }
    }
    /// Bit 22 - Receive enable acknowledge flag
    pub fn reack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 21 - Transmit enable acknowledge flag
    pub fn teack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 20 - Wakeup from Stop mode flag
    pub fn wuf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 19 - Receiver wakeup from Mute mode
    pub fn rwu(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 18 - Send break flag
    pub fn sbkf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - character match flag
    pub fn cmf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 16 - Busy flag
    pub fn busy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 15 - Auto baud rate flag
    pub fn abrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 14 - Auto baud rate error
    pub fn abre(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - End of block flag
    pub fn eobf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Receiver timeout
    pub fn rtof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 10 - CTS flag
    pub fn cts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - CTS interrupt flag
    pub fn ctsif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - LIN break detection flag
    pub fn lbdf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 7 - Transmit data register empty
    pub fn txe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Transmission complete
    pub fn tc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 5 - Read data register not empty
    pub fn rxne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Idle line detected
    pub fn idle(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Overrun error
    pub fn ore(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Noise detected flag
    pub fn nf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Framing error
    pub fn fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Parity error
    pub fn pe(&mut self, value: bool) -> &mut Self {
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
    register: ::volatile_register::RW<u32>,
}

impl Icr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut IcrW) -> &mut IcrW
    {
        let mut rw = IcrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> IcrR {
        IcrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: IcrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct IcrR {
    bits: u32,
}

impl IcrR {
    /// Bit 20 - Wakeup from Stop mode clear flag
    pub fn wucf(&self) -> bool {
        const OFFSET: u8 = 20;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 17 - Character match clear flag
    pub fn cmcf(&self) -> bool {
        const OFFSET: u8 = 17;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 12 - End of timeout clear flag
    pub fn eobcf(&self) -> bool {
        const OFFSET: u8 = 12;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 11 - Receiver timeout clear flag
    pub fn rtocf(&self) -> bool {
        const OFFSET: u8 = 11;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 9 - CTS clear flag
    pub fn ctscf(&self) -> bool {
        const OFFSET: u8 = 9;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 8 - LIN break detection clear flag
    pub fn lbdcf(&self) -> bool {
        const OFFSET: u8 = 8;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 6 - Transmission complete clear flag
    pub fn tccf(&self) -> bool {
        const OFFSET: u8 = 6;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 4 - Idle line detected clear flag
    pub fn idlecf(&self) -> bool {
        const OFFSET: u8 = 4;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 3 - Overrun error clear flag
    pub fn orecf(&self) -> bool {
        const OFFSET: u8 = 3;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 2 - Noise detected clear flag
    pub fn ncf(&self) -> bool {
        const OFFSET: u8 = 2;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 1 - Framing error clear flag
    pub fn fecf(&self) -> bool {
        const OFFSET: u8 = 1;
        self.bits & (1 << OFFSET) != 0
    }
    /// Bit 0 - Parity error clear flag
    pub fn pecf(&self) -> bool {
        const OFFSET: u8 = 0;
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
    /// Bit 20 - Wakeup from Stop mode clear flag
    pub fn wucf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 17 - Character match clear flag
    pub fn cmcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 12 - End of timeout clear flag
    pub fn eobcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 11 - Receiver timeout clear flag
    pub fn rtocf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 9 - CTS clear flag
    pub fn ctscf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 8 - LIN break detection clear flag
    pub fn lbdcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 6 - Transmission complete clear flag
    pub fn tccf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 4 - Idle line detected clear flag
    pub fn idlecf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 3 - Overrun error clear flag
    pub fn orecf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 2 - Noise detected clear flag
    pub fn ncf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 1 - Framing error clear flag
    pub fn fecf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    /// Bit 0 - Parity error clear flag
    pub fn pecf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

pub struct Rdr {
    register: ::volatile_register::RO<u32>,
}

impl Rdr {
    pub fn read(&self) -> RdrR {
        RdrR { bits: self.register.read() }
    }
}

#[derive(Clone, Copy)]
pub struct RdrR {
    bits: u32,
}

impl RdrR {
    /// Bits 0:8 - Receive data value
    pub fn rdr(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

#[derive(Clone, Copy)]
pub struct RdrW {
    bits: u32,
}

impl RdrW {
    /// Reset value
    pub fn reset_value() -> Self {
        RdrW { bits: 0 }
    }
    /// Bits 0:8 - Receive data value
    pub fn rdr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

pub struct Tdr {
    register: ::volatile_register::RW<u32>,
}

impl Tdr {
    pub fn modify<F>(&mut self, f: F)
        where F: FnOnce(&mut TdrW) -> &mut TdrW
    {
        let mut rw = TdrW { bits: self.register.read() };
        f(&mut rw);
        self.register.write(rw.bits);
    }
    pub fn read(&self) -> TdrR {
        TdrR { bits: self.register.read() }
    }
    pub fn write(&mut self, value: TdrW) {
        self.register.write(value.bits)
    }
}

#[derive(Clone, Copy)]
pub struct TdrR {
    bits: u32,
}

impl TdrR {
    /// Bits 0:8 - Transmit data value
    pub fn tdr(&self) -> u16 {
        const MASK: u32 = 511;
        const OFFSET: u8 = 0;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

#[derive(Clone, Copy)]
pub struct TdrW {
    bits: u32,
}

impl TdrW {
    /// Reset value
    pub fn reset_value() -> Self {
        TdrW { bits: 0 }
    }
    /// Bits 0:8 - Transmit data value
    pub fn tdr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0;
        const MASK: u16 = 511;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
