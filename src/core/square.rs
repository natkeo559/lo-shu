use std::fmt;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

use crate::order::Params;

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Square<P: Params>(pub [u8; P::ELEMENTS])
where
    [(); P::ELEMENTS]:;

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct GenericSquare<T: Clone + Copy, P: Params>(pub [T; P::ELEMENTS])
where
    [(); P::ELEMENTS]:;

impl<T: Clone + Copy, P: Params> GenericSquare<T, P>
where
    [(); P::ELEMENTS]:,
{
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the slice has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Swaps two elements in the Square.
    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    /// Returns a reference to an element or subslice depending on the type of index.
    pub fn get<I: std::slice::SliceIndex<[T]>>(&self, index: I) -> Option<&I::Output> {
        self.0.get(index)
    }

    /// Returns a reference to an element or subslice, without doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used
    pub unsafe fn get_unchecked<I: std::slice::SliceIndex<[T]>>(&self, index: I) -> &I::Output {
        self.0.get_unchecked(index)
    }

    ///Creates a Square from an array.
    pub fn from_array(array: [T; P::ELEMENTS]) -> Self {
        Self(array)
    }
}

impl<P: Params> Square<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the slice has a length of 0.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Swaps two elements in the Square.
    pub fn swap(&mut self, a: usize, b: usize) {
        self.0.swap(a, b);
    }

    /// Returns a reference to an element or subslice depending on the type of index.
    pub fn get<I: std::slice::SliceIndex<[u8]>>(&self, index: I) -> Option<&I::Output> {
        self.0.get(index)
    }

    /// Returns a reference to an element or subslice, without doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used
    pub unsafe fn get_unchecked<I: std::slice::SliceIndex<[u8]>>(&self, index: I) -> &I::Output {
        self.0.get_unchecked(index)
    }

    ///Creates a Square from an array.
    pub fn from_array(array: [u8; P::ELEMENTS]) -> Self {
        Self(array)
    }
}

impl<I, T: Clone + Copy, P: Params> std::ops::Index<I> for GenericSquare<T, P>
where
    I: std::slice::SliceIndex<[T]>,
    [(); P::ELEMENTS]:,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I, T: Clone + Copy, P: Params> std::ops::IndexMut<I> for GenericSquare<T, P>
where
    I: std::slice::SliceIndex<[T]>,
    [(); P::ELEMENTS]:,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<I, P: Params> std::ops::Index<I> for Square<P>
where
    I: std::slice::SliceIndex<[u8]>,
    [(); P::ELEMENTS]:,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I, P: Params> std::ops::IndexMut<I> for Square<P>
where
    I: std::slice::SliceIndex<[u8]>,
    [(); P::ELEMENTS]:,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T: Clone + Copy + Hash, P: Params> Hash for GenericSquare<T, P>
where
    [(); P::ELEMENTS]:,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<T: Copy + Clone + PartialEq, P: Params> PartialEq for GenericSquare<T, P>
where
    [(); P::ELEMENTS]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Copy + Clone + Eq, P: Params> Eq for GenericSquare<T, P> where [(); P::ELEMENTS]: {}

impl<P: Params> Hash for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

impl<P: Params> PartialEq for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<P: Params> Eq for Square<P> where [(); P::ELEMENTS]: {}

impl<P: Params> fmt::Display for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = self.0.into_iter();
        for i in 1..=P::ORDER {
            write!(f, "{:?}", a.by_ref().take(P::ORDER).collect_vec())?;
            if i != P::ORDER {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_square {
    use crate::{OrderFour, OrderThree};

    use super::*;

    #[test]
    fn test_square_from_array_3() {
        let a = Square::<OrderThree>::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(Square([1, 2, 3, 4, 5, 6, 7, 8, 9]), a);
        assert_eq!(9, a.len());
        assert_eq!(5, a[4]);
    }

    #[test]
    fn test_square_from_array_4() {
        let a = Square::<OrderFour>::from_array([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
        ]);

        assert_eq!(
            Square([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            a
        );
        assert_eq!(16, a.len());
        assert_eq!(5, a[4]);
        println!("{}", a)
    }
}
