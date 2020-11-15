use crate::{pixelindex::PixelIterator, Color, Effect, PixelIndex, PixelIndexable};
use core::marker::PhantomData;
use generic_array::{ArrayLength, GenericArray};
use num_traits::Float;
use palette::Hue;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rtt_target::{rprint, rprintln};

pub struct Flame<T: PixelIndexable>
where
    T::SIZE: ArrayLength<f32>,
{
    cells: GenericArray<f32, T::SIZE>,
    wind: f32,
    gust_duration: u32,
    flow_min: f32,
    flow_max: f32,
    cool_min: f32,
    cool_max: f32,
    fuel_min: f32,
    fuel_max: f32,
    heat_min: f32,
    heat_max: f32,
    hue_shift: f32,
    rng: SmallRng,
    _phantom: PhantomData<T>,
}

impl<T: PixelIndexable> Default for Flame<T>
where
    T::SIZE: ArrayLength<f32>,
{
    fn default() -> Self {
        let rng = SmallRng::seed_from_u64(1234);
        let mut cells = GenericArray::default();

        let px_iter: PixelIterator<T> = PixelIterator::all();
        for idx in px_iter {
            if idx.down().is_none() {
                cells[idx.usize()] = 1.0;
            }
        }
        Self {
            cells,
            wind: 0.0,
            gust_duration: 200,
            flow_max: 0.5,
            flow_min: 0.1,
            cool_max: 0.1,
            cool_min: 0.02,
            fuel_min: 0.95,
            fuel_max: 2.0,
            heat_min: 0.0,
            heat_max: 2.5,
            hue_shift: 70.0,
            rng,
            _phantom: PhantomData,
        }
    }
}

impl<T: PixelIndexable> Flame<T>
where
    T::SIZE: ArrayLength<f32>,
{
    fn flow(&mut self, from: PixelIndex<T>, to: PixelIndex<T>) {
        let val = self
            .rng
            .gen_range(self.flow_min, self.flow_max)
            .min(self.cells[from.usize()]);
        self.cells[from.usize()] -= val;
        self.cells[to.usize()] = (self.cells[to.usize()] + val).min(self.heat_max);
    }
    fn rise(&mut self, idx: PixelIndex<T>) {
        if let Some(up) = idx.up() {
            self.flow(idx, up);
            self.cool(up);
        }
    }
    fn blow(&mut self, idx: PixelIndex<T>) {
        if self.rng.gen_bool(self.wind.abs() as f64) {
            if let Some(src) = if self.wind < 0.0 {
                idx.left()
            } else {
                idx.right()
            } {
                self.flow(src, idx);
            }
        }
    }
    fn feed(&mut self, idx: PixelIndex<T>) {
        if idx.down().is_none() {
            let fuel = self.rng.gen_range(self.flow_min, self.flow_max);
            self.cells[idx.usize()] =
                (self.cells[idx.usize()].max(self.fuel_min) + fuel).min(self.fuel_max);
        }
    }
    fn cool(&mut self, idx: PixelIndex<T>) {
        if let Some(_) = idx.up() {
            let (_, height) = idx.as_spherical();
            if self.rng.gen_bool(height as f64) {
                let cool = self.rng.gen_range(self.cool_min, self.cool_max);
                self.cells[idx.usize()] = (self.cells[idx.usize()] - cool).max(0.0);
            }
        } else {
            self.cells[idx.usize()] = 0.0;
        }
    }
    fn debug_dump(&self) {
        if let Some(top) = T::index_top() {
            for (row_count, row) in top.iter_down().enumerate() {
                rprint!("{}: ", row_count);
                for (px_count, px) in row.iter_right().enumerate() {
                    rprint!("{:.1}, ", self.cells[px.usize()]);
                    if px_count > 20 {
                        rprintln!("\nRow overflow on row {}", row_count);
                        break;
                    }
                }
                rprint!("\n");
            }
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
            //self.cool(idx);
            self.blow(idx);
            self.rise(idx);
            self.feed(idx);
        }
        if self.rng.gen_ratio(1, self.gust_duration) {
            self.wind = self.rng.gen_range(-1.0, 1.0);
        }
        // Logging
        if self.rng.gen_ratio(1, 20) {
            self.debug_dump();
        }
    }

    fn render(&self, color: Color, model: &mut T) {
        for idx in model.iter_pixels() {
            let val = self.cells[idx.usize()].max(0.0).min(1.0);
            //let val = val * val;
            //*model.get_mut(idx) = color.darken(1.0 - val).desaturate(1.0 - val).clamp();
            //*model.get_mut(idx) = color.darken(val).desaturate(1.0 - val).clamp();
            if val > self.heat_min {
                *model.get_mut(idx) = Color {
                    l: color.l * val,
                    chroma: color.chroma * val,
                    ..color
                };
            } else {
                *model.get_mut(idx) = Color::new(0.0, 0.0, 0.0);
            }
            *model.get_mut(idx) = color.shift_hue(self.hue_shift * (1.0 - val));
        }
    }

    fn init(&mut self, model: &mut T) {
        for idx in model.iter_pixels() {
            self.cells[idx.usize()] = 0.0;
            *model.get_mut(idx) = Color::new(0.0, 0.0, 0.0);
        }
    }

    fn rotate_cw(&mut self, color: &mut Color) {
        *color = color.shift_hue(2.0);
        rprintln!("Hue: {}", color.hue.to_positive_degrees());
    }

    fn rotate_ccw(&mut self, color: &mut Color) {
        *color = color.shift_hue(-2.0);
        rprintln!("Hue: {}", color.hue.to_positive_degrees());
    }

    //fn click(&mut self, color: &mut Color) {}
}
