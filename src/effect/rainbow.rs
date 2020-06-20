use crate::{Color, Lantern, Unit};
use palette::Hue;

pub enum Orientation {
    Horizontal,
    Vertical,
    Spiral,
}

pub struct Rainbow {
    color: Color,
    speed: Unit,
    step: Unit,
    orient: Orientation,
}

impl Rainbow {
    pub fn new<T: Into<Unit>>(color: Color, speed: T, step: T) -> Self {
        let orient = Orientation::Spiral;
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
        self.color.shift_hue(self.speed);
        for angle in 0..20 {
            for height in 0..7 {
                let px = model.get_cylinder_pixel(angle, height);
                use Orientation::*;
                match self.orient {
                    Horizontal => {
                        *px = self
                            .color
                            .clone()
                            .shift_hue(self.step.wrapping_mul_int(angle as i32))
                    }
                    Vertical => {
                        *px = self
                            .color
                            .clone()
                            .shift_hue(self.step.wrapping_mul_int(height as i32))
                    }
                    Spiral => {
                        *px = self.color.clone().shift_hue(
                            self.step
                                .wrapping_mul_int(height as i32)
                                .wrapping_add(self.step.wrapping_mul_int(angle as i32)),
                        )
                    }
                }
            }
        }
    }
}
