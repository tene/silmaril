use crate::{
    hsv::{HSV, HUE_MAX},
    math::{noise::Simplex, Fix},
    Lantern,
};
use num_traits::float::FloatCore;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rtt_target::rprintln;
pub struct Storm {
    color: HSV,
    speed: f32,
    offset: f32,
    noise: Simplex,
}

impl Storm {
    pub fn new(color: HSV, speed: f32) -> Self {
        let noise = Simplex::new(137);
        let offset = 0.0;
        Self {
            color,
            speed,
            offset,
            noise,
        }
    }
    pub fn tick(&mut self, model: &mut Lantern) {
        for angle in 0..20 {
            for height in 0..7 {
                let x = angle as f32 * 256.0 / 20.0;
                let y = height as f32 * 256.0 / 7.0;
                let z = self.offset;
                let val = self.noise.noise_3d(x, y, z);
                //let val = self.noise.billow_3d(16, x, y, z, 0.5, 0.5);
                let hue = ((val + 1.0) * (HUE_MAX as f32) / 2.0) as i16;
                let val: u8 = ((val + 1.0) * 100.0) as u8;
                let px = model.get_cylinder_pixel(angle, height);
                *px = HSV::new(hue, 16, val);
            }
        }
        self.offset += self.speed;
    }
}
