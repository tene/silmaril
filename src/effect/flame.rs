use crate::{pixelindex::PixelIterator, Color, Effect, PixelIndexable};
use core::marker::PhantomData;
use generic_array::{ArrayLength, GenericArray};
use num_traits::Float;
use palette::{Limited, Saturate, Shade};
use rand::{rngs::SmallRng, Rng, SeedableRng};

pub struct Flame<T: PixelIndexable>
where
    T::SIZE: ArrayLength<f32>,
{
    cells: GenericArray<f32, T::SIZE>,
    wind: f32,
    rng: SmallRng,
    _phantom: PhantomData<T>,
}

impl<T: PixelIndexable> Default for Flame<T>
where
    T::SIZE: ArrayLength<f32>,
{
    fn default() -> Self {
        let rng = SmallRng::seed_from_u64(1234);
        Self {
            cells: GenericArray::default(),
            wind: 0.0,
            rng,
            _phantom: PhantomData,
        }
    }
}

impl<T: PixelIndexable> Effect<T> for Flame<T>
where
    T::SIZE: ArrayLength<f32>,
{
    fn tick(&mut self, _color: &mut Color) {
        let px_iter: PixelIterator<T> = PixelIterator::all();
        for idx in px_iter {
            match idx.down() {
                Some(src) => {
                    self.cells[idx.usize()] =
                        (self.cells[src.usize()] - self.rng.gen_range(0.0, 0.35)).max(0.0);
                }
                None => {
                    self.cells[idx.usize()] = self.rng.gen_range(0.875, 1.0);
                }
            }
            if self.rng.gen_bool(self.wind.abs() as f64) {
                if let Some(src) = if self.wind < 0.0 {
                    idx.left()
                } else {
                    idx.right()
                } {
                    let x = self.rng.gen_range(0.0, 0.25);
                    let a = (self.cells[src.usize()] - x).max(0.0);
                    let b = self.cells[src.usize()] - a;
                    self.cells[src.usize()] = a;
                    self.cells[idx.usize()] += b;
                }
            }
        }
        if self.rng.gen_ratio(1, 200) {
            self.wind = self.rng.gen_range(-1.0, 1.0);
        }
    }

    fn render(&self, color: Color, model: &mut T) {
        for idx in model.iter_pixels() {
            let val = self.cells[idx.usize()];
            *model.get_mut(idx) = color.darken(1.0 - val).desaturate(1.0 - val).clamp();
        }
    }

    fn init(&mut self, model: &mut T) {
        for idx in model.iter_pixels() {
            self.cells[idx.usize()] = 0.0;
            *model.get_mut(idx) = Color::new(0.0, 0.0, 0.0);
        }
    }
}