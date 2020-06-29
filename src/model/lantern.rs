use crate::{lch_to_rgb, pixelindex::*, Color};
use palette::{Hue, Saturate, Shade};

pub struct Lantern {
    pub color: Color,
    pub pixels: [Color; 125],
}

impl Lantern {
    pub fn new(color: Color) -> Self {
        let pixels = [color; 125];
        Self { color, pixels }
    }
    pub fn render(&mut self, buf: &mut [[u8; 3]; 125]) {
        for (&src, dst) in self.pixels.iter().zip(buf.iter_mut()) {
            *dst = lch_to_rgb(src);
        }
    }
    pub fn clear(&mut self) {
        self.pixels = [self.color; 125];
    }
    pub fn darken<T: Into<f32> + Copy>(&mut self, d: T) {
        for px in self.pixels.iter_mut() {
            *px = px.darken(d.into());
        }
    }
    pub fn saturate<T: Into<f32> + Copy>(&mut self, d: T) {
        for px in self.pixels.iter_mut() {
            *px = px.saturate(d.into());
        }
    }
    pub fn shift_hue_all<T: Into<f32> + Copy>(&mut self, d: T) {
        for px in self.pixels.iter_mut() {
            *px = px.shift_hue(d.into());
        }
    }
    pub fn get_cylinder_pixel(&mut self, angle: u8, height: u8) -> &mut Color {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LanternFace {
    North,
    East,
    South,
    West,
    Top,
}

impl PixelIndexable for Lantern {
    type Face = LanternFace;
    const SIZE: usize = 125;
    const FACES: usize = 5;
    fn get(&self, idx: PixelIndex<Self>) -> Color {
        self.pixels[idx]
    }
    fn get_mut(&mut self, idx: PixelIndex<Self>) -> &mut Color {
        &mut self.pixels[idx]
    }
    fn index_above(_idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        todo!()
    }
    fn index_below(_idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        todo!()
    }
    fn index_left(_idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        todo!()
    }
    fn index_right(_idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        todo!()
    }
    fn index_to_face(_idx: PixelIndex<Self>) -> Self::Face {
        todo!()
    }
    fn index_to_spherical(idx: PixelIndex<Self>) -> (f32, f32) {
        let i: usize = idx.into();
        let face = i / 25;
        let face_offset = i.rem_euclid(25);
        let x = face_offset.rem_euclid(5);
        let y = face_offset / 5;
        if face < 4 {
            // XXX TODO offset angle by 0.5 to match top
            let angle = (face * 5 + x) as f32 / 20.0;
            let height = (4 - y) as f32 / 7.0;
            (angle, height)
        } else {
            let angle = TOP_IDX_ANGLE[face_offset];
            let height = (7 - FACE_IDX_RADIUS[face_offset]) as f32 / 7.0;
            (angle, height)
        }
    }
}

const FACE_IDX_RADIUS: [usize; 25] = [
    2, 2, 2, 2, 2, 2, 1, 1, 1, 2, 2, 1, 0, 1, 2, 2, 1, 1, 1, 2, 2, 2, 2, 2, 2,
];

const TOP_IDX_ANGLE: [f32; 25] = [
    8.0 / 16.0,
    7.0 / 16.0,
    6.0 / 16.0,
    5.0 / 16.0,
    4.0 / 16.0,
    9.0 / 16.0,
    4.0 / 9.0,
    3.0 / 9.0,
    2.0 / 9.0,
    3.0 / 16.0,
    10.0 / 16.0,
    5.0 / 9.0,
    0.0,
    1.0 / 9.0,
    2.0 / 16.0,
    11.0 / 16.0,
    6.0 / 9.0,
    7.0 / 9.0,
    0.0 / 9.0,
    1.0 / 16.0,
    12.0 / 16.0,
    13.0 / 16.0,
    14.0 / 16.0,
    15.0 / 16.0,
    0.0 / 16.0,
];
