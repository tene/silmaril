use crate::hsv::HSV;
use smart_leds::RGB8;

pub struct Lantern {
    pub color: HSV,
    pub pixels: [HSV; 125],
}

impl Lantern {
    pub fn new(color: HSV) -> Self {
        let pixels = [color; 125];
        Self { color, pixels }
    }
    pub fn render(&mut self, buf: &mut [RGB8; 125]) {
        for (src, dst) in self.pixels.iter().zip(buf.iter_mut()) {
            *dst = src.to_rgb().into();
        }
    }
    pub fn clear(&mut self) {
        self.pixels = [self.color; 125];
    }
    pub fn shift_value_all(&mut self, d: i16) {
        for px in self.pixels.iter_mut() {
            px.shift_val_sat(d);
        }
    }
    pub fn shift_saturation_all(&mut self, d: i16) {
        for px in self.pixels.iter_mut() {
            px.shift_saturation_sat(d);
        }
    }
    pub fn shift_hue_all(&mut self, d: i16) {
        for px in self.pixels.iter_mut() {
            px.shift_hue(d);
        }
    }
    pub fn get_cylinder_pixel(&mut self, angle: u8, height: u8) -> &mut HSV {
        let face = (angle / 5).rem_euclid(4) as usize;
        let angle = angle.rem_euclid(20) as usize;
        let x = angle - (face * 5);
        if height < 5 {
            let base: usize = face * 25;
            let y = 4 - height as usize;
            let offset = x + (5 * y);
            let index = base + offset;
            if index > 100 {
                panic!(
                    "Out of bounds pixel\nangle: {}, height: {}\nx: {}, y: {}\n\nindex: {}",
                    angle, height, x, y, index
                );
            }
            return self.pixels.get_mut(index).unwrap();
        // face
        } else {
            let angle = 19 - (angle + 10).rem_euclid(20);
            let base = 100;
            let r = 7usize.saturating_sub(height as usize);
            let offset = match r {
                0 => 13,
                1 => [
                    7, 7, 8, 9, 9, 9, 9, 14, 19, 19, 19, 19, 18, 17, 17, 17, 17, 12, 9, 9,
                ][angle],
                2 => [
                    1, 2, 3, 4, 5, 5, 10, 15, 20, 25, 25, 24, 23, 22, 21, 21, 16, 11, 6, 1,
                ][angle],
                _ => unreachable!(),
            } - 1;
            return self.pixels.get_mut(base + offset).unwrap();
        }
    }
}
