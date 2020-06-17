use crate::{hsv::HSV, Lantern};
use itertools::izip;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use smart_leds::RGB8;

pub mod drops;
pub mod rainbow;
pub mod solid;

pub use drops::Drops;
pub use rainbow::Rainbow;
pub use solid::Solid;

// Some pixels are faster than others
pub struct Demo2 {
    speed: i16,
    deviation: [i16; 125],
    state: [HSV; 125],
}

impl Demo2 {
    pub fn new(init: HSV, speed: i16, max_deviation: i16) -> Self {
        let mut rng = SmallRng::seed_from_u64(1234);
        let mut deviation = [0; 125];
        for i in deviation.iter_mut() {
            *i = rng.gen_range(0, max_deviation);
        }
        let state = [init; 125];
        Self {
            speed,
            deviation,
            state,
        }
    }
    pub fn tick(&mut self, model: &mut [RGB8; 125]) {
        for (item, extra, target) in izip!(
            self.state.iter_mut(),
            self.deviation.iter(),
            model.iter_mut()
        ) {
            item.shift_hue(self.speed + extra);
            *target = item.to_rgb().into();
        }
    }
}

#[derive(Copy, Clone)]
pub struct Demo1 {
    count: u8,
    color: HSV,
    offset: i16,
    stride: u8,
}

impl Demo1 {
    pub fn new(count: u8, color: HSV, offset: i16) -> Self {
        Self {
            count,
            color,
            offset,
            stride: 5,
        }
    }
}

impl Iterator for Demo1 {
    type Item = HSV;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            return None;
        }
        self.count -= 1;
        if self.stride == 0 {
            self.color.shift_hue(self.offset);
            self.stride = 4;
        } else {
            self.stride -= 1;
        }
        if self.stride % 2 != 0 {
            Some(self.color.shifted_hue(self.offset))
        } else {
            Some(self.color)
        }
    }
}
