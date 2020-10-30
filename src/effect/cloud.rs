use crate::{math::noise::Simplex, Color, Effect, PixelIndexable};
use core::marker::PhantomData;
//use num_traits::float::FloatCore;
//use rand::rngs::SmallRng;
//use rand::{Rng, SeedableRng};
//use rtt_target::rprintln;
pub struct Cloud<T: PixelIndexable> {
    speed: f32,
    offset: f32,
    noise: Simplex,
    _pd: PhantomData<T>,
}

impl<T: PixelIndexable> Cloud<T> {
    pub fn default() -> Self {
        let speed = 10f32;
        let noise = Simplex::new(137);
        let offset = 0.0.into();
        let _pd = PhantomData;
        Self {
            speed,
            offset,
            noise,
            _pd,
        }
    }
}

impl<T: PixelIndexable> Effect<T> for Cloud<T> {
    fn tick(&mut self, _color: &mut Color) {
        self.offset += self.speed;
    }
    fn render(&self, color: Color, model: &mut T) {
        for idx in model.iter_pixels() {
            let (dir, height) = idx.as_spherical();
            let x = dir * 256.0;
            let y = height * 256.0;
            let z = self.offset;
            let val = self.noise.noise_3d(x, y, z);
            //let val = self.noise.billow_3d(4, x, y, z, 0.5, 0.5);
            let l = (val + 1.0) * 50.0;
            *model.get_mut(idx) = Color { l, ..color };
        }
    }
}
