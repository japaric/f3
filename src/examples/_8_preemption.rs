//! Preemptive multitasking
//!
//! Same as the resource example but the `roulette` task now has higher
//! priority.
//!
//! ```
//! 
//! #![feature(const_fn)]
//! #![feature(used)]
//! #![no_std]
//! 
//! // version = "0.2.0", default-features = false
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
//! use rtfm::{C0, C1, C16, C2, Local, P0, P1, P2, Resource};
//! 
//! #[derive(Clone, Copy)]
//! enum Direction {
//!     Clockwise,
//!     Counterclockwise,
//! }
//! 
//! struct Command {
//!     direction: Cell<Direction>,
//!     reset: Cell<bool>,
//! }
//! 
//! impl Command {
//!     const fn new() -> Command {
//!         Command {
//!             direction: Cell::new(Direction::Clockwise),
//!             reset: Cell::new(false),
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
//!         ceiling: C2,
//!     },
//!     USART1: Peripheral {
//!         register_block: Usart1,
//!         ceiling: C1,
//!     },
//! });
//! 
//! static COMMAND: Resource<Command, C2> = Resource::new(Command::new());
//! 
//! // INITIALIZATION PHASE
//! fn init(ref prio: P0, ceil: &C16) {
//!     let gpioa = GPIOA.access(prio, ceil);
//!     let gpioe = GPIOE.access(prio, ceil);
//!     let rcc = RCC.access(prio, ceil);
//!     let tim7 = TIM7.access(prio, ceil);
//!     let timer = Timer(&tim7);
//!     let usart1 = USART1.access(prio, ceil);
//! 
//!     led::init(&gpioe, &rcc);
//!     timer.init(&rcc, FREQUENCY);
//!     Serial(&usart1).init(&gpioa, &rcc, BAUD_RATE);
//! 
//!     timer.resume();
//! }
//! 
//! // IDLE LOOP
//! fn idle(_prio: P0, _ceil: C0) -> ! {
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
//!         priority: P2,
//!         enabled: true,
//!     },
//!     receive: Task {
//!         interrupt: Usart1Exti25,
//!         priority: P1,
//!         enabled: true,
//!     },
//! });
//! 
//! fn receive(mut task: Usart1Exti25, ref prio: P1, ref ceil: C1) {
//!     static BUFFER: Local<Vec<u8, [u8; 16]>, Usart1Exti25> = {
//!         Local::new(Vec::new([0; 16]))
//!     };
//! 
//!     let usart1 = USART1.access(prio, ceil);
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
//!                 b"reverse" => {
//!                     ceil.raise(
//!                         &COMMAND, |ceil| {
//!                             let command = COMMAND.access(prio, ceil);
//! 
//!                             let Command { ref direction, .. } = *command;
//! 
//!                             match direction.get() {
//!                                 Direction::Clockwise => {
//!                                     direction.set(Direction::Counterclockwise)
//!                                 }
//!                                 Direction::Counterclockwise => {
//!                                     direction.set(Direction::Clockwise)
//!                                 }
//!                             }
//!                         }
//!                     );
//!                 }
//!                 b"reset" => {
//!                     ceil.raise(
//!                         &COMMAND, |ceil| {
//!                             let command = COMMAND.access(prio, ceil);
//! 
//!                             command.reset.set(true);
//!                         }
//!                     );
//!                 }
//!                 _ => {}
//!             }
//! 
//!             buffer.clear();
//!         } else {
//!             if buffer.push(byte).is_err() {
//!                 // TODO proper error handling
//!                 // for now we just clear the buffer when full
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
//! fn roulette(mut task: Tim7, ref prio: P2, ref ceil: C2) {
//!     static STATE: Local<u8, Tim7> = Local::new(0);
//! 
//!     let tim7 = TIM7.access(prio, ceil);
//!     let timer = Timer(&tim7);
//! 
//!     if timer.clear_update_flag().is_ok() {
//!         let state = STATE.borrow_mut(&mut task);
//!         let curr = *state;
//! 
//!         let command = COMMAND.access(prio, ceil);
//!         let direction = command.direction.get();
//!         let Command { ref reset, .. } = *command;
//! 
//!         let n = u8(LEDS.len()).unwrap();
//!         let next = if reset.get() {
//!             reset.set(false);
//!             0
//!         } else {
//!             match direction {
//!                 Direction::Clockwise => (curr + 1) % n,
//!                 Direction::Counterclockwise => {
//!                     curr.checked_sub(1).unwrap_or(n - 1)
//!                 }
//!             }
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
