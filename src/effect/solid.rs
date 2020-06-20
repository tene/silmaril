use crate::{Color, Lantern, Unit};
use palette::Hue;

pub struct Solid {
    color: Color,
    speed: Unit,
}

impl Solid {
    pub fn new(color: Color, speed: Unit) -> Self {
        Self { color, speed }
    }
    pub fn tick(&mut self, model: &mut Lantern) {
        self.color = self.color.shift_hue(self.speed);
        for angle in 0..20 {
            for height in 0..7 {
                let px = model.get_cylinder_pixel(angle, height);
                *px = self.color;
            }
        }
    }
}
