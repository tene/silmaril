use crate::{Color, Effect, Lantern, PixelIndex, PixelIndexable};
use core::marker::PhantomData;
use palette::Hue;

pub enum Orientation {
    Horizontal,
    Vertical,
    Spiral,
}

pub struct Rainbow<T: PixelIndexable> {
    pub color: Color,
    speed: f32,
    step: f32,
    orient: Orientation,
    _pd: PhantomData<T>,
}

impl<T: PixelIndexable> Rainbow<T> {
    pub fn new<F: Into<f32>>(color: Color, speed: F, step: F) -> Self {
        let orient = Orientation::Spiral;
        let speed = speed.into();
        let step = step.into();
        Self {
            color,
            speed,
            step,
            orient,
            _pd: PhantomData,
        }
    }
}

impl<T: PixelIndexable> Effect<T> for Rainbow<T> {
    fn tick(&mut self) {
        self.color = self.color.shift_hue(self.speed);
    }
    fn render(&self, model: &mut T) {
        for idx in model.iter_pixels() {
            let (dir, height) = idx.as_spherical();
            use Orientation::*;
            *model.get_mut(idx) = match self.orient {
                Horizontal => self.color.shift_hue(self.step * dir),
                Vertical => self.color.shift_hue(self.step * height),
                Spiral => self
                    .color
                    .shift_hue(self.step * height / 2.0 + self.step * dir),
            };
        }
    }
}
