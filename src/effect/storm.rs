use crate::{math::noise::Simplex, Color, Lantern, Unit};
use palette::Hue;
//use num_traits::float::FloatCore;
//use rand::rngs::SmallRng;
//use rand::{Rng, SeedableRng};
//use rtt_target::rprintln;
pub struct Storm {
    color: Color,
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
                //let val = self.noise.billow_3d(16, x, y, z, 0.5, 0.5);
                let hue: Unit = ((val + 1.0) / 2.0).into();
                let val = ((val + 1.0) / 2.0).into();
                let px = model.get_cylinder_pixel(angle, height);
                *px = Color {
                    l: val,
                    ..self.color.shift_hue(hue)
                };
            }
        }
        self.offset += self.speed;
    }
}
