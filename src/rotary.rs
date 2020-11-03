// Mostly copied from https://crates.io/crates/rotary-encoder-hal
// Local fork to handle interrupts
use embedded_hal as hal;
use hal::digital::v2::InputPin;
use stm32f4xx_hal::gpio::ExtiPin;

/// Holds current/old state and both [`InputPin`](https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html)
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rotary<A, B, C> {
    pin_a: A,
    pin_b: B,
    pin_c: C,
    state: u8,
    released: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Into<f32> for Direction {
    fn into(self) -> f32 {
        match self {
            Direction::Clockwise => 1.0,
            Direction::CounterClockwise => -1.0,
        }
    }
}

impl core::ops::Mul<f32> for Direction {
    type Output = f32;

    fn mul(self, rhs: f32) -> Self::Output {
        let lhs: f32 = self.into();
        lhs.mul(rhs)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Click {
    Press,
    Release,
}

impl<A, B, C> Rotary<A, B, C>
where
    A: InputPin + ExtiPin,
    B: InputPin + ExtiPin,
    C: InputPin + ExtiPin,
    A::Error: core::fmt::Debug,
    B::Error: core::fmt::Debug,
    C::Error: core::fmt::Debug,
{
    /// Accepts three `InputPin`s, these will be read on every `update()`
    /// [InputPin]: https://docs.rs/embedded-hal/0.2.3/embedded_hal/digital/v2/trait.InputPin.html
    pub fn new(pin_a: A, pin_b: B, pin_c: C) -> Self {
        Self {
            pin_a,
            pin_b,
            pin_c,
            state: 0u8,
            released: true,
        }
    }
    // XXX TODO Maybe this should return InputEvents?
    pub fn update(&mut self) -> (Option<Direction>, Option<Click>) {
        let dir = self.update_dir();
        let click = self.update_click();
        (dir, click)
    }
    /// Call `update` to evaluate the next state of the encoder, propagates errors from `InputPin` read
    fn update_dir(&mut self) -> Option<Direction> {
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
    fn update_click(&mut self) -> Option<Click> {
        let rv = match (self.released, self.pin_c.is_high().unwrap()) {
            (false, true) => {
                self.released = true;
                Some(Click::Release)
            }
            (true, false) => {
                self.released = false;
                Some(Click::Press)
            }
            _ => None,
        };
        self.pin_c.clear_interrupt_pending_bit();
        rv
    }
}
