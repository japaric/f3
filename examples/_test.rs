// This is actually a test that runs on the HOST (!). I had to hack things around quite a bit to
// make this work; that's way this ended up in this directory :-).

extern crate f3;

use f3::peripheral;

macro_rules! offset {
    ($register:expr, $offset:expr) => {
        if $offset != &$register as *const _ as usize {
            panic!("{}: expected 0x{:x}, got 0x{:x}. Off by {}",
                   stringify!($register), $offset, &$register as *const _ as usize,
                   (&$register as *const _ as isize - $offset).abs())
        }
    }
}

fn main() {
    let dbgmcu = unsafe { &*(0x0 as *const peripheral::dbgmcu::Registers) };

    offset!(dbgmcu.idcode, 0x0);
    offset!(dbgmcu.cr, 0x4);
    offset!(dbgmcu.apb1fz, 0x8);
    offset!(dbgmcu.apb2fz, 0xc);

    let gpio = unsafe { &*(0x0 as *const peripheral::gpio::Registers) };

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

    let rcc = unsafe { &*(0x0 as *const peripheral::rcc::Registers) };

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

    let tim = unsafe { &*(0x0 as *const peripheral::tim::Registers) };

    offset!(tim.cr1, 0x0);
    offset!(tim.cr2, 0x4);
    offset!(tim.dier, 0xc);
    offset!(tim.sr, 0x10);
    offset!(tim.egr, 0x14);
    offset!(tim.cnt, 0x24);
    offset!(tim.psc, 0x28);
    offset!(tim.arr, 0x2c);

    let usart = unsafe { &*(0x0 as *const peripheral::usart::Registers) };

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
