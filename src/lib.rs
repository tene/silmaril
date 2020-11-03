#![no_std]
pub mod color;
pub mod effect;
pub use effect::Effect;
pub mod hsv;
pub use color::{lch_color, lch_to_rgb, Color};
pub mod model;
pub use model::lantern::Lantern;
pub mod math;
pub mod pixelindex;
pub use pixelindex::{FaceType, PixelIndex, PixelIndexable};
pub mod rotary;
pub use rotary::{Direction, Rotary};

pub enum Knobs {
    Knob1,
    Knob2,
    Knob3,
}
// XXX TODO Parameterize knob type
// pub enum InputEvent<K> {
pub enum InputEvent {
    Spin(Knobs, Direction),
    Press(Knobs),
    Release(Knobs),
}
