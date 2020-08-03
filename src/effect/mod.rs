use crate::pixelindex::PixelIndexable;

pub mod cloud;
pub mod drops;
pub mod rainbow;
pub mod solid;
pub mod sparks;
pub mod storm;

pub use cloud::Cloud;
pub use drops::Drops;
pub use rainbow::Rainbow;
pub use solid::Solid;
pub use sparks::Sparks;
pub use storm::Storm;

pub trait Effect<T: PixelIndexable> {
    fn tick(&mut self);
    fn render(&self, model: &mut T);
    fn rotate_cw(&mut self) {}
    fn rotate_ccw(&mut self) {}
    fn click(&mut self) {}
    //fn init(&mut self, model: &mut T) {}
    // XXX TODO input / control channels
}
