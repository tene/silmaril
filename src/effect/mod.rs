use crate::pixelindex::PixelIndexable;

pub mod cloud;
pub mod drops;
pub mod rainbow;
pub mod solid;
pub mod storm;

pub use cloud::Cloud;
pub use drops::Drops;
pub use rainbow::Rainbow;
pub use solid::Solid;
pub use storm::Storm;

pub trait Effect<T: PixelIndexable> {
    fn tick(&mut self) {}
    fn render(&self, model: &mut T);
    //fn init(&mut self, model: &mut T) {}
    // XXX TODO input / control channels
}
