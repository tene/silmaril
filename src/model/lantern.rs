use crate::{lch_to_rgb, pixelindex::*, Color, FaceType};
use num_traits::Float;
use palette::{Hue, Saturate, Shade};
use typenum::{U125, U5};
// XXX TODO Rename to Cube
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
    // TODO s/cylindrical/spherical
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
    type SIZE = U125;
    type FACES = U5;
    fn get(&self, idx: PixelIndex<Self>) -> Color {
        self.pixels[idx]
    }
    fn get_mut(&mut self, idx: PixelIndex<Self>) -> &mut Color {
        &mut self.pixels[idx]
    }
    /* Top Indices
    20 15 10 5 0
    21 16 11 6 1
    22 17 12 7 2
    23 18 13 8 3
    24 19 14 9 4
    */
    fn index_above(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        if idx.usize() >= 100 {
            let face_idx = idx.usize() - 100;
            if face_idx == 12 {
                return None;
            }
            let top_idx = [
                6, 7, 7, 7, 8, //
                11, 12, 12, 12, 13, //
                11, 12, 12, 12, 13, //
                11, 12, 12, 12, 13, //
                16, 17, 17, 17, 18,
            ][face_idx];
            return Some((top_idx + 100).into());
        }
        if idx.usize() % 25 < 5 {
            let face_idx = idx.usize() % 25;
            let top_idx = match idx.usize() / 25 {
                0 => [24, 19, 14, 9, 4][face_idx],
                1 => 4 - face_idx,
                2 => face_idx * 5,
                3 => 20 + face_idx,
                _ => unreachable!(),
            };
            Some((top_idx + 100).into())
        } else {
            Some(idx - 5)
        }
    }
    /* Top Indices
    20 15 10 5 0
    21 16 11 6 1
    22 17 12 7 2
    23 18 13 8 3
    24 19 14 9 4
    */
    fn index_below(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        if idx.usize() >= 100 {
            let face_idx = idx.usize() - 100;
            let idx = [
                29, 28, 27, 26, 4, 51, 100, 102, 104, 3, 52, 110, 113, 114, 2, 53, 120, 122, 124,
                1, 75, 76, 77, 78, 0,
            ][face_idx];
            return Some(idx.into());
        }
        if idx.usize() % 25 >= 20 {
            None
        } else {
            Some(idx + 5)
        }
    }
    fn index_left(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        if idx.usize() >= 100 {
            let face_idx = idx.usize() - 100;
            if face_idx == 12 {
                return None;
            }
            let idx = [
                1, 2, 3, 4, 9, 0, 7, 8, 13, 14, 5, 6, 12, 18, 19, 10, 11, 16, 17, 24, 15, 20, 21,
                22, 23,
            ][face_idx]
                + 100;
            return Some(idx.into());
        }
        let face_idx = idx.usize() % 25;
        if face_idx % 5 == 0 {
            let idx = (idx.usize() + 79) % 100;
            Some(idx.into())
        } else {
            Some(idx - 1)
        }
    }
    /* Top Indices
    20 15 10 5 0
    21 16 11 6 1
    22 17 12 7 2
    23 18 13 8 3
    24 19 14 9 4
    */
    fn index_right(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>> {
        if idx.usize() >= 100 {
            let face_idx = idx.usize() - 100;
            if face_idx == 12 {
                return None;
            }
            let idx = [
                5, 0, 1, 2, 3, 10, 11, 6, 7, 4, 15, 16, 12, 8, 9, 20, 17, 18, 13, 14, 21, 22, 23,
                24, 19,
            ][face_idx]
                + 100;
            return Some(idx.into());
        }
        let face_idx = idx.usize() % 25;
        if face_idx % 5 == 4 {
            let idx = (idx.usize() + 21) % 100;
            Some(idx.into())
        } else {
            Some(idx + 1)
        }
    }
    fn index_to_face(idx: PixelIndex<Self>) -> Self::Face {
        use LanternFace::*;
        [South, East, North, West, Top][idx.usize() / 25]
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
    fn index_to_row_col(_idx: PixelIndex<Self>) -> (usize, usize) {
        todo!()
    }
    fn index_to_face_type(idx: PixelIndex<Self>) -> FaceType {
        match Self::index_to_face(idx) {
            LanternFace::Top => FaceType::Top,
            _ => FaceType::Side,
        }
    }
    fn cylindrical_to_index(dir: f32, height: f32) -> PixelIndex<Self> {
        let x = ((dir % 1.0) * 19.99).trunc() as usize;
        let y = (height * 5.0).trunc().min(4.0).max(0.0) as usize;
        let face_offset = (x / 5) * 25;
        let row_offset = (4 - y) * 5;
        let col_offset = x % 5;
        (face_offset + row_offset + col_offset).into()
    }
    fn spherical_to_index(dir: f32, height: f32) -> PixelIndex<Self> {
        if height < 5.0 / 8.0 {
            return Self::cylindrical_to_index(dir, height * 8.0 / 5.0);
        }
        let r = 7 - (height * 8.0).trunc().min(7.0) as usize;
        /* Top Indices
        20 15 10 5 0
        21 16 11 6 1
        22 17 12 7 2
        23 18 13 8 3
        24 19 14 9 4
        */
        let top_id = match r {
            0 => 12,
            1 => {
                let dir_index = ((dir * 8.0) % 8.0).trunc() as usize;
                [18, 13, 8, 7, 6, 11, 16, 17][dir_index]
            }
            2 => {
                let dir_index = ((dir * 16.0) % 16.0).trunc() as usize;
                [24, 19, 14, 9, 4, 3, 2, 1, 0, 5, 10, 15, 20, 21, 22, 23][dir_index]
            }
            _ => unreachable!(),
        };
        (100 + top_id).into()
    }

    fn index_top() -> Option<PixelIndex<Self>> {
        Some(112.into())
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
