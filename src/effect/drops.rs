use crate::{
    hsv::{HSV, HUE_MAX},
    Lantern,
};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

const NUM_DROPS: usize = 16;
// TODO const generics
pub struct Drops {
    drops: [(HSV, u8, u8); NUM_DROPS],
    rng: SmallRng,
}

impl Drops {
    pub fn new(color: HSV) -> Self {
        let mut rng = SmallRng::seed_from_u64(1234);
        let mut drops = [(color, 0, 7); NUM_DROPS];
        for drop in drops.iter_mut() {
            drop.0.h = rng.gen_range(0, HUE_MAX as u16);
            //drop.0.s = 0;
            drop.1 = rng.gen_range(0, 20);
        }
        Self { drops, rng }
    }
    pub fn tick(&mut self, model: &mut Lantern) {
        model.shift_value_all(-2);
        model.shift_saturation_all(8);
        //model.shift_hue_all(50);
        for (color, angle, height) in self.drops.iter_mut() {
            if self
                .rng
                .gen_ratio(1 + ((color.h * 6) / HUE_MAX as u16) as u32, 10)
            {
                if *height == 0 {
                    *height = 7;
                    *angle = self.rng.gen_range(0, 20);
                    color.h = self.rng.gen_range(0, HUE_MAX as u16);
                //color.s = 0;
                //color.s = self.rng.gen_range(0, 255);
                } else {
                    *height -= 1;
                }
            }
            let px = model.get_cylinder_pixel(*angle, *height);
            *px = *color;
        }
    }
}
