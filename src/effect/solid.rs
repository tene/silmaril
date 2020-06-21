use crate::{Color, Lantern};
use palette::Hue;

pub struct Solid {
    color: Color,
    speed: f32,
}

impl Solid {
    pub fn new<T: Into<f32>>(color: Color, speed: T) -> Self {
        Self {
            color,
            speed: speed.into(),
        }
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
