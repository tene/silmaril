use crate::{hsv::HSV, Lantern};

pub struct Solid {
    color: HSV,
    speed: i16,
}

impl Solid {
    pub fn new(color: HSV, speed: i16) -> Self {
        Self { color, speed }
    }
    pub fn tick(&mut self, model: &mut Lantern) {
        self.color.shift_hue(self.speed);
        for angle in 0..20 {
            for height in 0..7 {
                let px = model.get_cylinder_pixel(angle, height);
                *px = self.color;
            }
        }
    }
}
