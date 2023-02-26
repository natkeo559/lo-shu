#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Square<T: Clone + Copy, const N: usize>(pub [T; N]);

impl<T: Clone + Copy, const N: usize> Square<T, N> {
    /// Returns the number of elements in the Square.
    pub fn len(&mut self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the slice has a length of 0.
    pub fn is_empty(&mut self) -> bool {
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
}

impl<T: Clone + Copy, const N: usize> Square<T, N> {
    ///Creates a Square from an array.
    pub fn from_array(array: [T; N]) -> Square<T, N> {
        Square(array)
    }
}

impl<I, T: Clone + Copy, const N: usize> std::ops::Index<I> for Square<T, N>
where
    I: std::slice::SliceIndex<[T]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I, T: Clone + Copy, const N: usize> std::ops::IndexMut<I> for Square<T, N>
where
    I: std::slice::SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod test_square3 {
    use crate::{OrderThree, Params};

    use super::*;

    #[test]
    fn test_square() {
        let a = Square::<u8, { OrderThree::ELEMENTS }>::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("{:?}", a)
    }
}
