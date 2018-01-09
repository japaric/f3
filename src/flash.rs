use stm32f30x::{flash, FLASH};

pub trait FlashExt {
    fn constraint(self) -> Parts;
}

impl FlashExt for FLASH {
    fn constraint(self) -> Parts {
        Parts {
            ACR: ACR { _0: () },
        }
    }
}

#[allow(non_snake_case)]
pub struct Parts {
    pub ACR: ACR,
}

pub struct ACR {
    _0: (),
}

impl ACR {
    pub(crate) fn acr(&mut self) -> &flash::ACR {
        // NOTE(unsafe) this proxy grants exclusive access to this register
        unsafe { &(*FLASH::ptr()).acr }
    }
}
