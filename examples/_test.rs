// This is actually a test that runs on the HOST (!). I had to hack things
// around quite a bit to make this work; that's way this ended up in this
// directory :-).

extern crate f3;

use f3::peripheral;

macro_rules! offset {
    ($register:expr, $offset:expr) => {
        if $offset != &$register as *const _ as usize {
            panic!("{}: expected 0x{:x}, got 0x{:x}. Off by {}",
                   stringify!($register), $offset,
                   &$register as *const _ as usize,
                   (&$register as *const _ as isize - $offset).abs())
        }
    }
}

fn main() {
    let btim = unsafe { &*(0x0 as *const peripheral::btim::BTim) };

    offset!(btim.cr1, 0x0);
    offset!(btim.cr2, 0x4);
    offset!(btim.dier, 0xc);
    offset!(btim.sr, 0x10);
    offset!(btim.egr, 0x14);
    offset!(btim.cnt, 0x24);
    offset!(btim.psc, 0x28);
    offset!(btim.arr, 0x2c);

    let dbgmcu = unsafe { &*(0x0 as *const peripheral::dbgmcu::Dbgmcu) };

    offset!(dbgmcu.idcode, 0x0);
    offset!(dbgmcu.cr, 0x4);
    offset!(dbgmcu.apb1fz, 0x8);
    offset!(dbgmcu.apb2fz, 0xc);

    let gpio = unsafe { &*(0x0 as *const peripheral::gpio::Gpio) };

    offset!(gpio.moder, 0x0);
    offset!(gpio.otyper, 0x4);
    offset!(gpio.ospeedr, 0x8);
    offset!(gpio.pupdr, 0xc);
    offset!(gpio.idr, 0x10);
    offset!(gpio.odr, 0x14);
    offset!(gpio.bsrr, 0x18);
    offset!(gpio.lckr, 0x1c);
    offset!(gpio.afrl, 0x20);
    offset!(gpio.afrh, 0x24);
    offset!(gpio.brr, 0x28);

    let gptim = unsafe { &*(0x0 as *const peripheral::gptim::GpTim) };

    offset!(gptim.cr1, 0x0);
    offset!(gptim.cr2, 0x4);
    offset!(gptim.smcr, 0x8);
    offset!(gptim.dier, 0xc);
    offset!(gptim.sr, 0x10);
    offset!(gptim.egr, 0x14);
    offset!(gptim.ccmr1_output, 0x18);
    offset!(gptim.ccmr2_output, 0x1c);
    offset!(gptim.ccer, 0x20);
    offset!(gptim.cnt, 0x24);
    offset!(gptim.psc, 0x28);
    offset!(gptim.arr, 0x2c);
    offset!(gptim.ccr1, 0x34);
    offset!(gptim.ccr2, 0x38);
    offset!(gptim.ccr3, 0x3c);
    offset!(gptim.ccr4, 0x40);
    offset!(gptim.dcr, 0x48);
    offset!(gptim.dmar, 0x4c);

    let i2c = unsafe { &*(0x0 as *const peripheral::i2c::I2c) };

    offset!(i2c.cr1, 0x0);
    offset!(i2c.cr2, 0x4);
    offset!(i2c.oar1, 0x8);
    offset!(i2c.oar2, 0xc);
    offset!(i2c.timingr, 0x10);
    offset!(i2c.timeoutr, 0x14);
    offset!(i2c.isr, 0x18);
    offset!(i2c.icr, 0x1c);
    offset!(i2c.pecr, 0x20);
    offset!(i2c.rxdr, 0x24);
    offset!(i2c.txdr, 0x28);

    let rcc = unsafe { &*(0x0 as *const peripheral::rcc::Rcc) };

    offset!(rcc.cr, 0x0);
    offset!(rcc.cfgr, 0x4);
    offset!(rcc.cir, 0x8);
    offset!(rcc.apb2rstr, 0xc);
    offset!(rcc.apb1rstr, 0x10);
    offset!(rcc.ahbenr, 0x14);
    offset!(rcc.apb2enr, 0x18);
    offset!(rcc.apb1enr, 0x1c);
    offset!(rcc.bdcr, 0x20);
    offset!(rcc.csr, 0x24);
    offset!(rcc.ahbrstr, 0x28);
    offset!(rcc.cfgr2, 0x2c);
    offset!(rcc.cfgr3, 0x30);

    let spi = unsafe { &*(0x0 as *const peripheral::spi::Spi) };

    offset!(spi.cr1, 0x00);
    offset!(spi.cr2, 0x04);
    offset!(spi.sr, 0x08);
    offset!(spi.dr, 0x0C);
    offset!(spi.crcpr, 0x10);
    offset!(spi.rxcrcr, 0x14);
    offset!(spi.txcrcr, 0x18);
    offset!(spi.i2scfgr, 0x1C);
    offset!(spi.i2spr, 0x20);

    let usart = unsafe { &*(0x0 as *const peripheral::usart::Usart) };

    offset!(usart.cr1, 0x0);
    offset!(usart.cr2, 0x4);
    offset!(usart.cr3, 0x8);
    offset!(usart.brr, 0xc);
    offset!(usart.gtpr, 0x10);
    offset!(usart.rtor, 0x14);
    offset!(usart.rqr, 0x18);
    offset!(usart.isr, 0x1c);
    offset!(usart.icr, 0x20);
    offset!(usart.rdr, 0x24);
    offset!(usart.tdr, 0x28);
}
