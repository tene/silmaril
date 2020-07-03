use crate::{math::noise::Simplex, Color, Effect, PixelIndexable};
use core::marker::PhantomData;
use rand::{rngs::SmallRng, Rng, SeedableRng};
//use palette::{Hue, Saturate};

const NUM_DROPS: usize = 16;
pub struct Storm<T: PixelIndexable> {
    _pd: PhantomData<T>,
    speed: f32,
    offset: f32,
    noise: Simplex,
    rng: SmallRng,
}

impl<T: PixelIndexable> Storm<T> {
    pub fn new(speed: f32) -> Self {
        let mut rng = SmallRng::seed_from_u64(1234);
        let noise = Simplex::new(137);
        let _pd = PhantomData;
        let offset = 0.0;
        Self {
            _pd,
            speed,
            offset,
            noise,
            rng,
        }
    }
}

impl<T: PixelIndexable> Effect<T> for Storm<T> {
    fn render(&self, model: &mut T) {
        for idx in model.iter_pixels() {
            let (dir, height) = idx.as_spherical();
            let x = dir * 256.0;
            let y = height * 2.0 + self.offset;
            let val = self.noise.noise_2d(x, y);
            let l = (val + 1.0) * 50.0;
            let color = Color::new(l, 100.0, 300.0);
            *model.get_mut(idx) = color;
        }
    }
    fn tick(&mut self) {
        self.offset += self.speed;
    }
}
