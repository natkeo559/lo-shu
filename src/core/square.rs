use crate::params::Params;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Square<T: Clone + Copy, P: Params>(pub [T; P::ELEMENTS])
where
    [(); P::ELEMENTS]:;

impl<T: Clone + Copy, P: Params> Square<T, P>
where
    [(); P::ELEMENTS]:,
{
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

impl<T: Clone + Copy, P: Params> Square<T, P>
where
    [(); P::ELEMENTS]:,
{
    ///Creates a Square from an array.
    pub fn from_array(array: [T; P::ELEMENTS]) -> Square<T, P> {
        Square(array)
    }
}

impl<I, T: Clone + Copy, P: Params> std::ops::Index<I> for Square<T, P>
where
    I: std::slice::SliceIndex<[T]>,
    [(); P::ELEMENTS]:,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I, T: Clone + Copy, P: Params> std::ops::IndexMut<I> for Square<T, P>
where
    I: std::slice::SliceIndex<[T]>,
    [(); P::ELEMENTS]:,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[cfg(test)]
mod test_square3 {
    use crate::OrderThree;

    use super::*;

    #[test]
    fn test_square() {
        let a = Square::<u8, OrderThree>::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        println!("{:?}", a)
    }
}
