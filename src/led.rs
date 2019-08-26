//! On-board user LEDs

use core::ops;

use embedded_hal::digital::v2::OutputPin;

use hal::gpio::gpioe::{self, PEx, PE10, PE11, PE12, PE13, PE14, PE15, PE8, PE9};
use hal::gpio::{Output, PushPull};

///  North LED
pub type LD3 = PE9<Output<PushPull>>;

/// Northeast LED
pub type LD5 = PE10<Output<PushPull>>;

/// East LED
pub type LD7 = PE11<Output<PushPull>>;

/// Southeast LED
pub type LD9 = PE12<Output<PushPull>>;

/// South LED
pub type LD10 = PE13<Output<PushPull>>;

/// Southwest LED
pub type LD8 = PE14<Output<PushPull>>;

/// West LED
pub type LD6 = PE15<Output<PushPull>>;

/// Northwest LED
pub type LD4 = PE8<Output<PushPull>>;

/// Cardinal directions. Each one matches one of the user LEDs.
pub enum Direction {
    /// North / LD3
    North,
    /// Northeast / LD5
    Northeast,
    /// East / LD7
    East,
    /// Southeast / LD9
    Southeast,
    /// South / LD10
    South,
    /// Southwest / LD8
    Southwest,
    /// West / LD6
    West,
    /// Northwest / LD4
    Northwest,
}

/// Array of all the user LEDs on the board
pub struct Leds {
    leds: [Led; 8],
}

impl Leds {
    /// Initializes all the user LEDs
    pub fn new(mut gpioe: gpioe::Parts) -> Self {
        let n = gpioe
            .pe9
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let ne = gpioe
            .pe10
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let e = gpioe
            .pe11
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let se = gpioe
            .pe12
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let s = gpioe
            .pe13
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let sw = gpioe
            .pe14
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let w = gpioe
            .pe15
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);
        let nw = gpioe
            .pe8
            .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

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

/// One of the on-board user LEDs
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
    /// Turns the LED off
    pub fn off(&mut self) -> core::result::Result<(),()> {
        self.pex.set_low()
    }

    /// Turns the LED on
    pub fn on(&mut self) -> core::result::Result<(),()> {
        self.pex.set_high()
    }
}
