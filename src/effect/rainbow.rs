use crate::{Color, Effect, PixelIndexable};
use core::marker::PhantomData;
use palette::{Hue, Saturate};

#[derive(Clone, Copy)]
pub enum Orientation {
    Horizontal,
    Vertical,
    Spiral,
}

impl Orientation {
    pub fn next(self) -> Self {
        use Orientation::*;
        match self {
            Horizontal => Vertical,
            Vertical => Spiral,
            Spiral => Horizontal,
        }
    }
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
        //self.color = self.color.shift_hue(self.speed);
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
                    .shift_hue(self.step * height / 2.0 + self.step * dir)
                    .saturate(height * 3.0),
            };
        }
    }
    fn rotate_cw(&mut self) {
        self.color = self.color.shift_hue(self.speed);
    }
    fn rotate_ccw(&mut self) {
        self.color = self.color.shift_hue(self.speed * -1.0);
    }
    fn click(&mut self) {
        self.orient = self.orient.next();
    }
}
