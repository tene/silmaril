use crate::Color;
use core::marker::PhantomData;

pub enum FaceType {
    Side,
    Top,
}
pub enum PixelIterator<T: PixelIndexable> {
    All(PixelIndex<T>),
    Column(Option<PixelIndex<T>>),
}

impl<T: PixelIndexable> PixelIterator<T> {
    pub fn all() -> Self {
        Self::All(0.into())
    }
}

impl<T: PixelIndexable> Iterator for PixelIterator<T> {
    type Item = PixelIndex<T>;
    fn next(&mut self) -> Option<Self::Item> {
        use PixelIterator::*;
        match self {
            All(next) => {
                let blah: usize = (*next).into();
                if blah >= T::SIZE {
                    None
                } else {
                    let rv = Some(*next);
                    *next += 1;
                    rv
                }
            }
            Column(px) => match px.take() {
                Some(rv) => {
                    *px = rv.down();
                    Some(rv)
                }
                None => None,
            },
        }
    }
}

pub trait PixelIndexable: Sized {
    type Face;
    const SIZE: usize;
    const FACES: usize;
    fn get(&self, idx: PixelIndex<Self>) -> Color;
    fn get_mut(&mut self, idx: PixelIndex<Self>) -> &mut Color;
    fn get_cylindrical_mut(&mut self, dir: f32, height: f32) -> &mut Color {
        let idx = Self::cylindrical_to_index(dir, height);
        self.get_mut(idx)
    }
    fn get_spherical_mut(&mut self, dir: f32, height: f32) -> &mut Color {
        let idx = Self::spherical_to_index(dir, height);
        self.get_mut(idx)
    }
    fn iter_pixels(&self) -> PixelIterator<Self> {
        PixelIterator::all()
    }
    fn column_iter_mut(&self, col: f32) -> PixelIterator<Self> {
        PixelIterator::Column(Some(Self::cylindrical_to_index(col, 1.0)))
    }
    fn cylindrical_to_index(dir: f32, height: f32) -> PixelIndex<Self>;
    fn spherical_to_index(dir: f32, height: f32) -> PixelIndex<Self>;
    fn index_to_face(idx: PixelIndex<Self>) -> Self::Face;
    fn index_to_face_type(idx: PixelIndex<Self>) -> FaceType;
    fn set_all(&mut self, color: Color) {
        for px in self.iter_pixels() {
            *(self.get_mut(px)) = color;
        }
    }
    fn map_pixels<F: Fn(PixelIndex<Self>, Color) -> Color>(&mut self, f: F) {
        for idx in self.iter_pixels() {
            *self.get_mut(idx) = f(idx, self.get(idx));
        }
    }
    /*
    fn index_to_cylindrical(idx: PixelIndex<Self>) -> (f32, f32, f32);
    fn index_to_face_xy(idx: PixelIndex<Self>) -> (Self::Face, f32, f32);
    fn index_to_cube_xyz(idx: PixelIndex<Self>) -> (f32, f32, f32);
    fn index_to_face_polar(idx: PixelIndex<Self>) -> (Self::Face, f32, f32);
    */
    fn index_to_spherical(idx: PixelIndex<Self>) -> (f32, f32);
    fn index_to_row_col(idx: PixelIndex<Self>) -> (usize, usize);

    // XXX TODO Should this be Option?  Wrapping variant?
    fn index_above(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>>;
    fn index_below(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>>;
    fn index_left(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>>;
    fn index_right(idx: PixelIndex<Self>) -> Option<PixelIndex<Self>>;
    /*
    fn index_rotate_x(idx: PixelIndex<Self>, turns: f32) -> Option<PixelIndex<Self>>;
    fn index_rotate_y(idx: PixelIndex<Self>, turns: f32) -> Option<PixelIndex<Self>>;
    fn index_rotate_z(idx: PixelIndex<Self>, turns: f32) -> Option<PixelIndex<Self>>;
    //fn iter_around(idx: PixelIndex<Self>) -> dyn Iterator<Item = PixelIndex<Self>>;
    //fn iter_neighbours
    */
}
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PixelIndex<T>(usize, PhantomData<T>);
impl<T> PixelIndex<T>
where
    T: PixelIndexable,
{
    pub fn as_spherical(self) -> (f32, f32) {
        T::index_to_spherical(self)
    }
    pub fn face(self) -> T::Face {
        T::index_to_face(self)
    }
    pub fn face_type(self) -> FaceType {
        T::index_to_face_type(self)
    }
    pub fn up(self) -> Option<Self> {
        T::index_above(self)
    }
    pub fn down(self) -> Option<Self> {
        T::index_below(self)
    }
    pub fn left(self) -> Option<Self> {
        T::index_left(self)
    }
    pub fn right(self) -> Option<Self> {
        T::index_right(self)
    }
    /*pub fn around(self) -> dyn Iterator<Item = Self> {
        T::iter_around(self)
    }*/
    pub fn usize(self) -> usize {
        self.0
    }
}

impl<T> Clone for PixelIndex<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T> Copy for PixelIndex<T> {}

impl<T> PixelIndex<T> {
    pub fn get(self, xs: &[Color]) -> Option<&Color> {
        xs.get(self.0 as usize)
    }
    pub fn get_mut(self, xs: &mut [Color]) -> Option<&mut Color> {
        xs.get_mut(self.0 as usize)
    }
}
impl<T> ::core::ops::Index<PixelIndex<T>> for [Color] {
    type Output = Color;
    fn index(&self, index: PixelIndex<T>) -> &Color {
        &self[index.0 as usize]
    }
}
impl<T> ::core::ops::IndexMut<PixelIndex<T>> for [Color] {
    fn index_mut(&mut self, index: PixelIndex<T>) -> &mut Color {
        &mut self[index.0 as usize]
    }
}
/*
// XXX TODO Heapless vec?
impl<T> ::core::ops::Index<PixelIndex<T>> for Vec<Color> {
    type Output = Color;
    fn index(&self, index: PixelIndex<T>) -> &Color {
        &self.as_slice()[index]
    }
}
impl<T> ::core::ops::IndexMut<PixelIndex<T>> for Vec<Color> {
    fn index_mut(&mut self, index: PixelIndex<T>) -> &mut Color {
        &mut self.as_mut_slice()[index]
    }
}
*/
impl<T> From<usize> for PixelIndex<T> {
    fn from(val: usize) -> PixelIndex<T> {
        PixelIndex(val, PhantomData)
    }
}
impl<T> From<PixelIndex<T>> for usize {
    fn from(val: PixelIndex<T>) -> usize {
        val.0
    }
}
impl<T> ::core::ops::Add<usize> for PixelIndex<T> {
    type Output = PixelIndex<T>;
    fn add(self, rhs: usize) -> PixelIndex<T> {
        PixelIndex(self.0 + rhs, PhantomData)
    }
}
impl<T> ::core::ops::AddAssign<usize> for PixelIndex<T> {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs
    }
}
impl<T> ::core::ops::Sub<usize> for PixelIndex<T> {
    type Output = PixelIndex<T>;
    fn sub(self, rhs: usize) -> PixelIndex<T> {
        PixelIndex(self.0 - rhs, PhantomData)
    }
}
impl<T> ::core::ops::SubAssign<usize> for PixelIndex<T> {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs
    }
}
impl<T> ::core::ops::Sub<PixelIndex<T>> for PixelIndex<T> {
    type Output = usize;
    fn sub(self, rhs: PixelIndex<T>) -> usize {
        self.0 - rhs.0
    }
}
