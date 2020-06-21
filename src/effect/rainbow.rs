use crate::{Color, Lantern};
use palette::Hue;

pub enum Orientation {
    Horizontal,
    Vertical,
    Spiral,
}

pub struct Rainbow {
    pub color: Color,
    speed: f32,
    step: f32,
    orient: Orientation,
}

impl Rainbow {
    pub fn new<T: Into<f32>>(color: Color, speed: T, step: T) -> Self {
        let orient = Orientation::Horizontal;
        let speed = speed.into();
        let step = step.into();
        Self {
            color,
            speed,
            step,
            orient,
        }
    }
    pub fn tick(&mut self, model: &mut Lantern) {
        self.color = self.color.shift_hue(self.speed);
        for angle in 0..20 {
            for height in 0..7 {
                let px = model.get_cylinder_pixel(angle, height);
                use Orientation::*;
                match self.orient {
                    Horizontal => *px = self.color.shift_hue(self.step * angle as f32),
                    Vertical => *px = self.color.shift_hue(self.step * height as f32),
                    Spiral => {
                        *px = self
                            .color
                            .shift_hue(self.step * height as f32 + self.step * angle as f32)
                    }
                }
            }
        }
    }
}
