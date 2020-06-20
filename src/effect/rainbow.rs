use crate::{hsv::HSV, Lantern};

pub enum Orientation {
    Horizontal,
    Vertical,
    Spiral,
}

pub struct Rainbow {
    color: HSV,
    speed: i16,
    step: i16,
    orient: Orientation,
}

impl Rainbow {
    pub fn new(color: HSV, speed: i16, step: i16) -> Self {
        let orient = Orientation::Spiral;
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
                    Horizontal => *px = self.color.shifted_hue(angle as i16 * self.step),
                    Vertical => *px = self.color.shifted_hue(height as i16 * self.step),
                    Spiral => {
                        *px = self
                            .color
                            .shifted_hue(height as i16 * self.step + angle as i16 * self.step)
                    }
                }
            }
        }
    }
}
