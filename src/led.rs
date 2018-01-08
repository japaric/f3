use gpio::GPIOE::{PE10, PE11, PE12, PE13, PE14, PE15, PE8, PE9};
use gpio::{Output, PushPull};

pub type LD3 = PE9<Output<PushPull>>;
pub type LD4 = PE8<Output<PushPull>>;
pub type LD5 = PE10<Output<PushPull>>;
pub type LD6 = PE15<Output<PushPull>>;
pub type LD7 = PE11<Output<PushPull>>;
pub type LD8 = PE14<Output<PushPull>>;
pub type LD9 = PE12<Output<PushPull>>;
pub type LD10 = PE13<Output<PushPull>>;
