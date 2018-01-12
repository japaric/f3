use core::ops;

use hal::prelude::*;

use hal::gpio::GPIOE::{PE10, PE11, PE12, PE13, PE14, PE15, PE8, PE9, PEx};
use hal::gpio::{Output, PushPull};

pub type LD3 = PE9<Output<PushPull>>;
pub type LD4 = PE8<Output<PushPull>>;
pub type LD5 = PE10<Output<PushPull>>;
pub type LD6 = PE15<Output<PushPull>>;
pub type LD7 = PE11<Output<PushPull>>;
pub type LD8 = PE14<Output<PushPull>>;
pub type LD9 = PE12<Output<PushPull>>;
pub type LD10 = PE13<Output<PushPull>>;

pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub struct Leds {
    leds: [Led; 8],
}

impl Leds {
    pub fn new(n: LD3, ne: LD5, e: LD7, se: LD9, s: LD10, sw: LD8, w: LD6, nw: LD4) -> Self {
        Leds {
            leds: [
                n.into(),
                ne.into(),
                e.into(),
                se.into(),
                s.into(),
                sw.into(),
                w.into(),
                nw.into(),
            ],
        }
    }
}

impl ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl ops::Index<Direction> for Leds {
    type Output = Led;

    fn index(&self, d: Direction) -> &Led {
        &self.leds[d as usize]
    }
}

impl ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

impl ops::IndexMut<Direction> for Leds {
    fn index_mut(&mut self, d: Direction) -> &mut Led {
        &mut self.leds[d as usize]
    }
}

pub struct Led {
    pex: PEx<Output<PushPull>>,
}

macro_rules! ctor {
    ($($ldx:ident),+) => {
        $(
            impl Into<Led> for $ldx {
                fn into(self) -> Led {
                    Led {
                        pex: self.downgrade(),
                    }
                }
            }
        )+
    }
}

ctor!(LD3, LD4, LD5, LD6, LD7, LD8, LD9, LD10);

impl Led {
    pub fn off(&mut self) {
        self.pex.set_low()
    }

    pub fn on(&mut self) {
        self.pex.set_high()
    }
}
