use stm32f30x::{rcc, RCC};

pub trait RccExt {
    fn split(self) -> Rcc;
}

impl RccExt for RCC {
    fn split(self) -> Rcc {
        Rcc {
            AHB: AHB { _0: () },
            APB1: APB1 { _0: () },
            APB2: APB2 { _0: () },
        }
    }
}

#[allow(non_snake_case)]
pub struct Rcc {
    pub AHB: AHB,
    pub APB1: APB1,
    pub APB2: APB2,
}

pub struct AHB {
    _0: (),
}

impl AHB {
    pub(crate) fn enr(&mut self) -> &rcc::AHBENR {
        unsafe { &(*RCC::ptr()).ahbenr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::AHBRSTR {
        unsafe { &(*RCC::ptr()).ahbrstr }
    }
}

pub struct APB1 {
    _0: (),
}

impl APB1 {
    pub(crate) fn enr(&mut self) -> &rcc::APB1ENR {
        unsafe { &(*RCC::ptr()).apb1enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB1RSTR {
        unsafe { &(*RCC::ptr()).apb1rstr }
    }
}

pub struct APB2 {
    _0: (),
}

impl APB2 {
    pub(crate) fn enr(&mut self) -> &rcc::APB2ENR {
        unsafe { &(*RCC::ptr()).apb2enr }
    }

    pub(crate) fn rstr(&mut self) -> &rcc::APB2RSTR {
        unsafe { &(*RCC::ptr()).apb2rstr }
    }
}
