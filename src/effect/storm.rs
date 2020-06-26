use crate::{math::noise::Simplex, Color, Lantern};
use palette::Hue;
//use num_traits::float::FloatCore;
//use rand::rngs::SmallRng;
//use rand::{Rng, SeedableRng};
//use rtt_target::rprintln;
pub struct Storm {
    pub color: Color,
    speed: f32,
    offset: f32,
    noise: Simplex,
}

impl Storm {
    pub fn new(color: Color, speed: f32) -> Self {
        let noise = Simplex::new(137);
        let offset = 0.0.into();
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
                //let val = self.noise.billow_3d(4, x, y, z, 0.5, 0.5);
                let hue: f32 = (val + 1.0) * 180.0;
                let val = 10.0 + (val + 1.0) * 20.0;
                let px = model.get_cylinder_pixel(angle, height);
                *px = Color {
                    l: val,
                    ..self.color.shift_hue(hue)
                };
                *px = Color::new(val, 0.0, 0.0);
            }
        }
        self.offset += self.speed;
    }
}
