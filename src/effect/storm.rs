use crate::{math::noise::Simplex, Color, Effect, PixelIndexable};
use core::marker::PhantomData;
use palette::{Mix, Shade};
use rand::{rngs::SmallRng, Rng, SeedableRng};

const NUM_DROPS: usize = 8;
pub struct Storm<T: PixelIndexable> {
    _pd: PhantomData<T>,
    bg_color: Color,
    drop_color: Color,
    drop_fade: f32,
    cloud_speed: f32,
    drop_speed: f32,
    offset: f32,
    strike_chance: f32,
    bolt: (f32, f32),
    bolt_fade: f32,
    noise: Simplex,
    drops: [(f32, f32, f32); NUM_DROPS],
    rng: SmallRng,
}

impl<T: PixelIndexable> Storm<T> {
    pub fn new(
        bg_color: Color,
        drop_color: Color,
        drop_fade: f32,
        cloud_speed: f32,
        drop_speed: f32,
        strike_chance: f32,
        bolt_fade: f32,
    ) -> Self {
        let mut rng = SmallRng::seed_from_u64(1234);
        let noise = Simplex::new(137);
        let _pd = PhantomData;
        let offset = 0.0;
        let mut drops = [(0.0, 0.0, 0.0); NUM_DROPS];
        for drop in drops.iter_mut() {
            *drop = (rng.gen_range(0.0, 1.0), rng.gen_range(0.0, 1.0), 0.0);
        }
        let bolt = (0.0, 0.0);
        Self {
            _pd,
            bg_color,
            drop_color,
            cloud_speed,
            drop_speed,
            drop_fade,
            offset,
            noise,
            bolt,
            strike_chance,
            bolt_fade,
            drops,
            rng,
        }
    }

    pub fn default() -> Self {
        let dim = Color::new(5.0, 5.0, 305.0);
        let drop = Color::new(0.0, 0.0, 305.0);
        Storm::new(dim, drop, 0.01, 0.05, 0.02, 0.15, 0.8)
    }
}

impl<T: PixelIndexable> Effect<T> for Storm<T> {
    fn render(&self, _color: Color, model: &mut T) {
        for idx in model.iter_pixels() {
            match idx.face_type() {
                crate::FaceType::Side => {
                    let px = model.get_mut(idx);
                    *px = px.mix(&self.bg_color, self.drop_fade);
                }
                crate::FaceType::Top => {
                    let (dir, height) = idx.as_spherical();
                    let x = dir * 256.0;
                    let y = height * 2.0 + self.offset;
                    let val = self.noise.noise_2d(x, y);
                    let l = (val + 1.0) * 25.0;
                    let color = Color::new(l, l, self.bg_color.hue);
                    *model.get_mut(idx) = color;
                }
            }
        }
        for &(dir, height, _speed) in &self.drops {
            *model.get_cylindrical_mut(dir, height) = self.drop_color;
        }
        if self.bolt.1 > 0.01 {
            for px in model.column_iter_mut(self.bolt.0) {
                *model.get_mut(px) = self
                    .bg_color
                    .mix(&Color::new(100.0, 0.0, self.bg_color.hue), self.bolt.1);
            }
        }
    }
    fn tick(&mut self, _color: &mut Color) {
        self.offset += self.cloud_speed;
        for drop in self.drops.iter_mut() {
            if drop.1 < 0.0 {
                *drop = (self.rng.gen_range(0.0, 1.0), 1.0, 0.0);
            } else {
                drop.1 -= drop.2;
                drop.2 += self.drop_speed;
            }
        }
        if self.rng.gen_bool(self.strike_chance as f64) {
            self.bolt = (self.rng.gen_range(0.0, 1.0), 1.0);
        } else {
            self.bolt.1 *= self.bolt_fade;
        }
    }
    fn rotate_cw(&mut self, _color: &mut Color) {
        //self.bg_color.chroma = 50.0;
        self.bg_color = self.bg_color.lighten(0.1);
    }
    fn rotate_ccw(&mut self, _color: &mut Color) {
        //self.bg_color.chroma = 0.0;
        self.bg_color = self.bg_color.darken(0.1);
    }
}
