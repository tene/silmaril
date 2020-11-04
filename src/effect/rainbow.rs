use crate::{Color, Effect, PixelIndexable};
use core::marker::PhantomData;
use palette::Hue;

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
    speed: f32,
    step: f32,
    orient: Orientation,
    _pd: PhantomData<T>,
}

impl<T: PixelIndexable> Rainbow<T> {
    pub fn new<F: Into<f32>>(speed: F, step: F) -> Self {
        let orient = Orientation::Spiral;
        let speed = speed.into();
        let step = step.into();
        Self {
            speed,
            step,
            orient,
            _pd: PhantomData,
        }
    }
    pub fn default() -> Self {
        Rainbow::new(10.0, 360.0)
    }
}

impl<T: PixelIndexable> Effect<T> for Rainbow<T> {
    fn tick(&mut self, color: &mut Color) {
        *color = color.shift_hue(self.speed);
    }
    fn render(&self, color: Color, model: &mut T) {
        for idx in model.iter_pixels() {
            let (dir, height) = idx.as_spherical();
            use Orientation::*;
            *model.get_mut(idx) = match self.orient {
                Horizontal => color.shift_hue(self.step * dir),
                Vertical => color.shift_hue(self.step * height),
                Spiral => color.shift_hue(self.step * height / 2.0 + self.step * dir),
            };
        }
    }
    fn rotate_cw(&mut self) {
        //self.color = self.color.shift_hue(self.speed);
        self.speed *= 1.1;
    }
    fn rotate_ccw(&mut self) {
        //self.color = self.color.shift_hue(self.speed * -1.0);
        self.speed *= 0.9;
    }
    fn click(&mut self) {
        self.orient = self.orient.next();
    }
}
