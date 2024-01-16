use std::fmt;
use std::hash::{Hash, Hasher};

use itertools::Itertools;

use crate::order::Params;
use crate::ParameterSetError;

use core::array::try_from_fn;

/// A generic Square struct parameterized by the type `P` implementing the `Params` trait.
/// This struct represents a square with data stored in an array of unsigned 32-bit integers.
///
/// # Examples
///
/// ```
/// use lo_shu::{Square, O3};
///
/// let a = Square::<O3>::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Square<P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub data: [u32; P::ELEMENTS],
}

impl<P: Params> Square<P>
where
    [(); P::ELEMENTS]:,
{
    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` if the slice has a length of 0.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Swaps two elements in the Square.
    pub fn swap(&mut self, a: usize, b: usize) {
        self.data.swap(a, b);
    }

    /// Returns a reference to an element or subslice depending on the type of index.
    pub fn get<I: std::slice::SliceIndex<[u32]>>(&self, index: I) -> Option<&I::Output> {
        self.data.get(index)
    }

    /// Returns a reference to an element or subslice, without doing bounds checking.
    ///
    /// # Safety
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting reference is not used
    pub unsafe fn get_unchecked<I: std::slice::SliceIndex<[u32]>>(&self, index: I) -> &I::Output {
        self.data.get_unchecked(index)
    }

    ///Creates a Square from an array.
    #[must_use]
    pub fn from_array(data: [u32; P::ELEMENTS]) -> Self {
        Self { data }
    }
}

impl<P: Params> TryFrom<&[u32]> for Square<P>
where
    [(); P::ELEMENTS]:,
{
    type Error = ParameterSetError;

    fn try_from(item: &[u32]) -> Result<Self, Self::Error>
    where
        [(); P::ELEMENTS]:,
    {
        let err =
            ParameterSetError::ElementCount("Item length does not match P::ELEMENTS!".to_string());

        if item.len() == P::ELEMENTS {
            let data: Result<[u32; P::ELEMENTS], std::array::TryFromSliceError> = item.try_into();
            match data {
                Ok(data) => Ok(Self { data }),
                Err(_) => Err(err),
            }
        } else {
            Err(err)
        }
    }
}

impl<I, P: Params> std::ops::Index<I> for Square<P>
where
    I: std::slice::SliceIndex<[u32]>,
    [(); P::ELEMENTS]:,
{
    type Output = I::Output;

    #[must_use]
    fn index(&self, index: I) -> &Self::Output {
        &self.data[index]
    }
}

impl<I, P: Params> std::ops::IndexMut<I> for Square<P>
where
    I: std::slice::SliceIndex<[u32]>,
    [(); P::ELEMENTS]:,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<P: Params> Hash for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<P: Params> PartialEq for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<P: Params> Eq for Square<P> where [(); P::ELEMENTS]: {}

impl<P: Params> fmt::Display for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut a = self.data.iter();
        for i in 1..=P::ORDER {
            write!(f, "{:?}", a.by_ref().take(P::ORDER).collect_vec())?;
            if i != P::ORDER {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

struct SquareDataVisitor<const N: usize>;

impl<'de, const N: usize> serde::de::Visitor<'de> for SquareDataVisitor<N> {
    type Value = [u32; N];

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a sequence of {N} u32")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        try_from_fn(|i| {
            seq.next_element()?
                .ok_or_else(|| serde::de::Error::invalid_length(i, &self))
        })
    }
}

impl<P: Params> serde::Serialize for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.data.serialize(serializer)
    }
}

impl<'de, P: Params> serde::Deserialize<'de> for Square<P>
where
    [(); P::ELEMENTS]:,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = deserializer.deserialize_seq(SquareDataVisitor)?;
        Ok(Self { data })
    }
}

#[cfg(test)]
mod test_square {
    use crate::{O3, O4};

    use super::*;

    #[test]
    fn test_square_from_array_3() {
        let a = Square::<O3>::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);

        assert_eq!(
            Square {
                data: [1, 2, 3, 4, 5, 6, 7, 8, 9]
            },
            a
        );
    }

    #[test]
    fn test_square_from_array_4() {
        let a = Square::<O4>::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);

        assert_eq!(
            Square {
                data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
            },
            a
        );
    }

    #[test]
    fn test_serde() -> Result<(), serde_json::Error> {
        let square = Square::<O3>::from_array([1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let expected = Square {
            data: [1, 2, 3, 4, 5, 6, 7, 8, 9],
        };
        let ser_square = serde_json::to_string(&square)?;
        let de_square: Square<O3> = serde_json::from_str(&ser_square)?;

        assert_eq!(ser_square, "[1,2,3,4,5,6,7,8,9]");
        assert_eq!(de_square, expected);

        Ok(())
    }
}
