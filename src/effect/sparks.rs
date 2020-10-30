use crate::{Color, Effect, PixelIndexable};
use core::marker::PhantomData;
use palette::{Hue, Mix};
use rand::{rngs::SmallRng, Rng, SeedableRng};

#[derive(Copy, Clone, Debug)]
struct Particle {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    color: Color,
}

impl Particle {
    fn new(x: f32, y: f32, dx: f32, dy: f32, color: Color) -> Self {
        Self {
            x,
            y,
            dx,
            dy,
            color,
        }
    }
    fn tick(&mut self, g: f32) {
        self.x = (self.x + self.dx + 1.0) % 1.0;
        self.y += self.dy;
        self.dy -= g;
    }
    fn shuffle(&mut self, rng: &mut SmallRng) {
        self.y = 0.0;
        self.dy = rng.gen_range(0.03, 0.04);
        self.dx = rng.gen_range(-0.01, 0.01);
        self.x = rng.gen_range(0.0, 1.0);
        self.color = Color::new(100.0, 50.0, rng.gen_range(0.0, 360.0));
    }
}

const NUM_SPARKS: usize = 2;

pub struct Sparks<T: PixelIndexable> {
    _pd: PhantomData<T>,
    fade: f32,
    shift: f32,
    sparks: [Particle; NUM_SPARKS],
    rng: SmallRng,
}

impl<T: PixelIndexable> Sparks<T> {
    pub fn new(fade: f32, shift: f32) -> Self {
        let _pd = PhantomData;
        let mut rng = SmallRng::seed_from_u64(137);
        let mut sparks = [Particle::new(
            0.0,
            0.0,
            0.0,
            0.0,
            Color::new(10.0, 10.0, rng.gen_range(0.0, 360.0)),
        ); NUM_SPARKS];
        for p in sparks.iter_mut() {
            p.shuffle(&mut rng);
        }
        Self {
            _pd,
            fade,
            shift,
            sparks,
            rng,
        }
    }
    pub fn default() -> Self {
        Self::new(0.2, 0.0)
    }
}

impl<T: PixelIndexable> Effect<T> for Sparks<T> {
    fn render(&self, _color: Color, model: &mut T) {
        model.map_pixels(|_idx, px| {
            px.mix(&Color::new(0.0, 0.0, px.hue), self.fade)
                .shift_hue(self.shift)
        });
        for p in &self.sparks {
            *model.get_spherical_mut(p.x, p.y) = p.color;
        }
    }
    fn tick(&mut self, _color: &mut Color) {
        for p in self.sparks.iter_mut() {
            p.tick(0.001);
            if p.dy <= 0.0 {
                p.shuffle(&mut self.rng);
            }
        }
    }
}
