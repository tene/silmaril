use crate::{Color, Effect, PixelIndexable};
use palette::Hue;

pub struct Solid {
    speed: f32,
}

impl Solid {
    pub fn new<T: Into<f32>>(speed: T) -> Self {
        Self {
            speed: speed.into(),
        }
    }
    pub fn default() -> Self {
        Self::new(1.0)
    }
}

impl<T: PixelIndexable> Effect<T> for Solid {
    fn tick(&mut self, color: &mut Color) {
        *color = color.shift_hue(self.speed);
    }

    fn render(&self, color: Color, model: &mut T) {
        model.set_all(color);
    }
}
