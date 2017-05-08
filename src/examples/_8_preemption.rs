//! Sharing memory using `Resource`
//!
//! This builds on top of the `concurrent` example. The `loopback` task now
//! additionally parses the received data as a command. Two commands are
//! available:
//!
//! - `reverse` - reverses the spin direction of the LED roulette
//! - `reset` - moves the roulette back to its start position (North)
//!
//! ```
//! 
//! #![feature(const_fn)]
//! #![feature(used)]
//! #![no_std]
//! 
//! // version = "0.2.2", default-features = false
//! extern crate cast;
//! 
//! // version = "0.2.0"
//! extern crate cortex_m_rt;
//! 
//! // version = "0.1.0"
//! #[macro_use]
//! extern crate cortex_m_rtfm as rtfm;
//! 
//! extern crate f3;
//! 
//! // version = "0.1.0"
//! extern crate heapless;
//! 
//! use core::cell::Cell;
//! 
//! use cast::{u8, usize};
//! use f3::led::{self, LEDS};
//! use f3::serial::Serial;
//! use f3::stm32f30x::interrupt::{Tim7, Usart1Exti25};
//! use f3::stm32f30x;
//! use f3::timer::Timer;
//! use heapless::Vec;
//! use rtfm::{C2, Local, P0, P1, P2, Resource, T0, T1, T2, TMax};
//! 
//! // SUPPORT CODE
//! #[derive(Clone, Copy)]
//! enum Direction {
//!     Clockwise,
//!     Counterclockwise,
//! }
//! 
//! impl Direction {
//!     fn reverse(self) -> Self {
//!         match self {
//!             Direction::Clockwise => Direction::Counterclockwise,
//!             Direction::Counterclockwise => Direction::Clockwise,
//!         }
//!     }
//! }
//! 
//! #[derive(Clone, Copy, PartialEq)]
//! enum Mode {
//!     Bounce,
//!     Continuous,
//! }
//! 
//! struct State {
//!     direction: Cell<Direction>,
//!     mode: Cell<Mode>,
//! }
//! 
//! impl State {
//!     const fn new() -> Self {
//!         State {
//!             direction: Cell::new(Direction::Clockwise),
//!             mode: Cell::new(Mode::Continuous),
//!         }
//!     }
//! }
//! 
//! // CONFIGURATION
//! pub const BAUD_RATE: u32 = 115_200; // bits per second
//! const FREQUENCY: u32 = 4; // Hz
//! 
//! // RESOURCES
//! peripherals!(stm32f30x, {
//!     GPIOA: Peripheral {
//!         register_block: Gpioa,
//!         ceiling: C0,
//!     },
//!     GPIOE: Peripheral {
//!         register_block: Gpioe,
//!         ceiling: C0,
//!     },
//!     RCC: Peripheral {
//!         register_block: Rcc,
//!         ceiling: C0,
//!     },
//!     TIM7: Peripheral {
//!         register_block: Tim7,
//!         ceiling: C2, // was `C1`
//!     },
//!     USART1: Peripheral {
//!         register_block: Usart1,
//!         ceiling: C1,
//!     },
//! });
//! 
//! // the ceiling was `C1`
//! static SHARED: Resource<State, C2> = Resource::new(State::new());
//! 
//! // INITIALIZATION PHASE
//! fn init(ref priority: P0, threshold: &TMax) {
//!     let gpioa = GPIOA.access(priority, threshold);
//!     let gpioe = GPIOE.access(priority, threshold);
//!     let rcc = RCC.access(priority, threshold);
//!     let tim7 = TIM7.access(priority, threshold);
//!     let timer = Timer(&tim7);
//!     let usart1 = USART1.access(priority, threshold);
//! 
//!     led::init(&gpioe, &rcc);
//!     timer.init(&rcc, FREQUENCY);
//!     Serial(&usart1).init(&gpioa, &rcc, BAUD_RATE);
//! 
//!     timer.resume();
//! }
//! 
//! // IDLE LOOP
//! fn idle(_priority: P0, _threshold: T0) -> ! {
//!     // Sleep
//!     loop {
//!         rtfm::wfi();
//!     }
//! }
//! 
//! // TASKS
//! tasks!(stm32f30x, {
//!     roulette: Task {
//!         interrupt: Tim7,
//!         priority: P2, // changed to `P2`
//!         enabled: true,
//!     },
//!     receive: Task {
//!         interrupt: Usart1Exti25,
//!         priority: P1,
//!         enabled: true,
//!     },
//! });
//! 
//! fn receive(mut task: Usart1Exti25, ref priority: P1, ref threshold: T1) {
//!     static BUFFER: Local<Vec<u8, [u8; 16]>, Usart1Exti25> = {
//!         Local::new(Vec::new([0; 16]))
//!     };
//! 
//!     let usart1 = USART1.access(priority, threshold);
//!     let serial = Serial(&usart1);
//! 
//!     if let Ok(byte) = serial.read() {
//!         if serial.write(byte).is_err() {
//!             // As we are echoing the bytes as soon as they arrive, it should
//!             // be impossible to have a TX buffer overrun
//!             #[cfg(debug_assertions)]
//!             unreachable!()
//!         }
//! 
//!         let buffer = BUFFER.borrow_mut(&mut task);
//! 
//!         if byte == b'r' {
//!             // end of command
//! 
//!             match &**buffer {
//!                 b"bounce" => {
//!                     threshold.raise(
//!                         &SHARED, |threshold| {
//!                             let shared = SHARED.access(priority, threshold);
//!                             shared.mode.set(Mode::Bounce)
//!                         }
//!                     );
//!                 }
//!                 b"continuous" => {
//!                     threshold.raise(
//!                         &SHARED, |threshold| {
//!                             let shared = SHARED.access(priority, threshold);
//!                             shared.mode.set(Mode::Continuous)
//!                         }
//!                     );
//!                 }
//!                 b"reverse" => {
//!                     threshold.raise(&SHARED, |threshold| {
//!                         let shared = SHARED.access(priority, threshold);
//!                         shared.direction.set(shared.direction.get().reverse());
//!                     });
//!                 }
//!                 _ => {}
//!             }
//! 
//!             buffer.clear();
//!         } else {
//!             if buffer.push(byte).is_err() {
//!                 // error: buffer full
//!                 // KISS: we just clear the buffer when it gets full
//!                 buffer.clear();
//!             }
//!         }
//!     } else {
//!         // Only reachable through `rtfm::request(receive)`
//!         #[cfg(debug_assertions)]
//!         unreachable!()
//!     }
//! }
//! 
//! fn roulette(mut task: Tim7, ref priority: P2, ref threshold: T2) {
//!     static STATE: Local<u8, Tim7> = Local::new(0);
//! 
//!     let tim7 = TIM7.access(priority, threshold);
//!     let timer = Timer(&tim7);
//! 
//!     if timer.clear_update_flag().is_ok() {
//!         let state = STATE.borrow_mut(&mut task);
//!         let curr = *state;
//! 
//!         let shared = SHARED.access(priority, threshold);
//!         let mut direction = shared.direction.get();
//! 
//!         if curr == 0 && shared.mode.get() == Mode::Bounce {
//!             direction = direction.reverse();
//!             shared.direction.set(direction);
//!         }
//! 
//!         let n = u8(LEDS.len()).unwrap();
//!         let next = match direction {
//!             Direction::Clockwise => (curr + 1) % n,
//!             Direction::Counterclockwise => curr.checked_sub(1).unwrap_or(n - 1),
//!         };
//! 
//!         LEDS[usize(curr)].off();
//!         LEDS[usize(next)].on();
//! 
//!         *state = next;
//!     } else {
//!         // Only reachable through `rtfm::request(roulette)`
//!         #[cfg(debug_assertion)]
//!         unreachable!()
//!     }
//! }
//! ```
// Auto-generated. Do not modify.
