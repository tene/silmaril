use crate::hsv::HSV;
use core::marker::PhantomData;

pub trait PixelIndexable {
    type Face;
    fn get(&self, idx: PixelIndex<Self>) -> HSV;
    fn get_mut(&mut self, idx: PixelIndex<Self>) -> &mut HSV;
}
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct PixelIndex<T: ?Sized>(usize, PhantomData<T>);
impl<T> PixelIndex<T> where T: PixelIndexable {}

impl<T> PixelIndex<T> {
    pub fn get(self, xs: &[HSV]) -> Option<&HSV> {
        xs.get(self.0 as usize)
    }
    pub fn get_mut(self, xs: &mut [HSV]) -> Option<&mut HSV> {
        xs.get_mut(self.0 as usize)
    }
}
impl<T> ::core::ops::Index<PixelIndex<T>> for [HSV] {
    type Output = HSV;
    fn index(&self, index: PixelIndex<T>) -> &HSV {
        &self[index.0 as usize]
    }
}
impl<T> ::core::ops::IndexMut<PixelIndex<T>> for [HSV] {
    fn index_mut(&mut self, index: PixelIndex<T>) -> &mut HSV {
        &mut self[index.0 as usize]
    }
}
/*
// XXX TODO Heapless vec?
impl<T> ::core::ops::Index<PixelIndex<T>> for Vec<HSV> {
    type Output = HSV;
    fn index(&self, index: PixelIndex<T>) -> &HSV {
        &self.as_slice()[index]
    }
}
impl<T> ::core::ops::IndexMut<PixelIndex<T>> for Vec<HSV> {
    fn index_mut(&mut self, index: PixelIndex<T>) -> &mut HSV {
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
