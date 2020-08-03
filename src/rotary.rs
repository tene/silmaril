// Mostly copied from https://crates.io/crates/rotary-encoder-hal
// Local fork to handle interrupts
use embedded_hal as hal;
use hal::digital::v2::InputPin;
use stm32f4xx_hal::gpio::ExtiPin;

/// Holds current/old state and both [`InputPin`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rotary<A, B> {
    pin_a: A,
    pin_b: B,
    state: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl<A, B> Rotary<A, B>
where
    A: InputPin + ExtiPin,
    B: InputPin + ExtiPin,
    A::Error: core::fmt::Debug,
    B::Error: core::fmt::Debug,
{
    /// Accepts two `InputPin`s, these will be read on every `update()`
    /// [InputPin]: https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html
    pub fn new(pin_a: A, pin_b: B) -> Self {
        Self {
            pin_a,
            pin_b,
            state: 0u8,
        }
    }
    /// Call `update` to evaluate the next state of the encoder, propagates errors from `InputPin` read
    pub fn update(&mut self) -> Option<Direction> {
        // use mask to get previous state value
        let mut s = self.state & 0b11;
        // move in the new state
        if self.pin_a.is_low().unwrap() {
            s |= 0b100;
        }
        if self.pin_b.is_low().unwrap() {
            s |= 0b1000;
        }
        self.pin_a.clear_interrupt_pending_bit();
        self.pin_b.clear_interrupt_pending_bit();

        // move new state in
        self.state = s >> 2;
        match s {
            0b0001 | 0b0111 | 0b1000 | 0b1110 => Some(Direction::Clockwise),
            0b0010 | 0b0100 | 0b1011 | 0b1101 => Some(Direction::CounterClockwise),
            _ => None,
        }
    }
}
