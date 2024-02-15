use crate::{order::Params, Square};
use crate::{Construction, Enumerable, EnumerableMarker, ParameterSetError, O3, O4, O5};
use std::array::try_from_fn;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq)]
pub enum Parity {
    Even,
    Odd,
}

impl fmt::Display for Parity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Parity::Even => {
                write!(f, "Even")?;
            }
            Parity::Odd => {
                write!(f, "Odd")?;
            }
        }

        Ok(())
    }
}

/// A struct representing a permutation based on parameters `P` implementing the `Params` trait.
/// It contains a square with data stored in an array of unsigned 32-bit integers.
///
/// # Examples
///
/// ```
/// use lo_shu::{Permutation, O4};
///
/// let a = Permutation::<O4>::identity();
/// ```
#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Permutation<P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<P>,
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    #[must_use]
    pub fn identity() -> Self {
        let mut data: [u32; P::ELEMENTS] = [0; P::ELEMENTS];
        for (elem, val) in data.iter_mut().zip(1..=P::ELEMENTS as u32) {
            *elem = val;
        }
        Self {
            square: Square { data },
        }
    }

    pub fn next_perm(&mut self) -> Option<&mut Self> {
        // Find non-increasing suffix
        let mut i: usize = P::ELEMENTS - 1;
        while i > 0 && self.square[i - 1] >= self.square[i] {
            i -= 1;
        }
        if i == 0 {
            return None;
        }

        // Find successor to pivot
        let mut j: usize = P::ELEMENTS - 1;
        while self.square[j] <= self.square[i - 1] {
            j -= 1;
        }
        self.square.swap(i - 1, j);

        // Reverse suffix
        self.square[i..].reverse();
        Some(self)
    }

    #[must_use]
    pub fn sign(&self) -> Parity
    where
        [(); P::ELEMENTS]:,
    {
        let mut inversions = 0;
        let data = self.square.data;
        for (x, px) in data.iter().enumerate() {
            for (y, py) in data.iter().enumerate() {
                if x < y && px > py {
                    inversions += 1;
                }
            }
        }

        if inversions % 2 == 0 {
            Parity::Even
        } else {
            Parity::Odd
        }
    }
}

impl<P: Params> Hash for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.square.hash(state);
    }
}

impl<P: Params> PartialEq for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}

impl<P: Params> Eq for Permutation<P> where [(); P::ELEMENTS]: {}

impl<P: Params> fmt::Display for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.square)?;

        Ok(())
    }
}

impl<P: Params> TryFrom<&[u32]> for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    type Error = ParameterSetError;

    fn try_from(slice: &[u32]) -> Result<Self, Self::Error> {
        let s = Square::<P>::try_from(slice)?;
        Ok(Permutation { square: s })
    }
}

impl<P: Params + EnumerableMarker> TryFrom<Construction<P>> for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    type Error = ParameterSetError;

    fn try_from(c: Construction<P>) -> Result<Self, Self::Error> {
        let vs = c.square.data.as_slice();
        let s = Square::<P>::try_from(vs)?;
        Ok(Self { square: s })
    }
}

/// Allow set-like types to order Permutations. This only works for Params that
/// implement Enumerable. Due to trait bound constraints, the Enumerable Params
/// are hard-coded via macro.
//-------------------------------------------------------------------------------------------------

macro_rules! impl_ord_for_enumerable_params {
    ($p:tt) => {
        impl Ord for Permutation<$p> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.index().cmp(&other.index())
            }
        }
    };
}

impl_ord_for_enumerable_params!(O3);
impl_ord_for_enumerable_params!(O4);
impl_ord_for_enumerable_params!(O5);

//-------------------------------------------------------------------------------------------------

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

impl<P: Params> serde::Serialize for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.square.serialize(serializer)
    }
}

impl<'de, P: Params> serde::Deserialize<'de> for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let data = deserializer.deserialize_seq(SquareDataVisitor)?;
        Ok(Self {
            square: Square::from_array(data),
        })
    }
}

#[cfg(test)]
mod test_perms {

    use std::collections::BTreeSet;

    use crate::{Enumerable, Parity, Permutation, Square, O3, O4};

    #[test]
    fn test_first_3() {
        let result: Permutation<O3> = Permutation {
            square: Square {
                data: [1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
        };
        let a = Permutation::<O3>::identity();
        assert_eq!(result, a);
        println!("{}", a)
    }

    #[test]
    fn test_first_4() {
        let result: Permutation<O4> = Permutation {
            square: Square {
                data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
            },
        };
        let a = Permutation::<O4>::identity();
        assert_eq!(result, a);
    }

    #[test]
    fn test_next_3() {
        let result: Permutation<O3> = Permutation {
            square: Square {
                data: [1, 2, 3, 4, 5, 6, 7, 9, 8],
            },
        };
        let mut a = Permutation::<O3>::identity();
        a = *a.next_perm().unwrap();
        assert_eq!(result, a);
    }

    #[test]
    fn test_next_4() {
        let result: Permutation<O4> = Permutation {
            square: Square {
                data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 15],
            },
        };
        let mut a = Permutation::<O4>::identity();
        a = *a.next_perm().unwrap();
        assert_eq!(result, a);
    }

    #[test]
    fn test_sign() {
        let odd_p1 = Permutation::<O4>::kth(1); // odd
        let odd_p2 = Permutation::<O4>::kth(2); // odd
        let even_p3 = Permutation::<O4>::kth(12); // even
        let even_p4 = Permutation::<O4>::kth(15); // even

        let odd_odd_p = odd_p1 * odd_p2;
        let even_even_p = even_p3 * even_p4;
        let odd_even_p = odd_p1 * even_p3;

        let odd_inv = odd_p1.inv();

        let even_inv = even_p3.inv();

        assert_eq!(odd_p1.sign(), Parity::Odd);
        assert_eq!(odd_p2.sign(), Parity::Odd);
        assert_eq!(even_p3.sign(), Parity::Even);
        assert_eq!(even_p4.sign(), Parity::Even);

        assert_eq!(odd_odd_p.sign(), Parity::Even);
        assert_eq!(even_even_p.sign(), Parity::Even);
        assert_eq!(odd_even_p.sign(), Parity::Odd);

        assert_eq!(even_inv.sign(), Parity::Even);
        assert_eq!(odd_inv.sign(), Parity::Odd);
    }

    #[test]
    fn test_serde() -> Result<(), serde_json::Error> {
        let perm = Permutation::<O3>::identity();
        let expected = Permutation {
            square: Square {
                data: [1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
        };
        let str: String = serde_json::to_string(&perm)?;
        let deser_square: Permutation<O3> = serde_json::from_str(&str)?;

        assert_eq!(str, "[1,2,3,4,5,6,7,8,9]");
        assert_eq!(deser_square, expected);

        Ok(())
    }

    #[test]
    fn test_serde_iter() -> Result<(), serde_json::Error> {
        let mut b = BTreeSet::new();
        let perm1 = Permutation::<O3>::identity();
        let perm2 = Permutation::<O3>::identity()
            .next_perm()
            .unwrap()
            .to_owned();

        b.insert(perm1);
        b.insert(perm2);

        let expected: BTreeSet<Permutation<O3>> = BTreeSet::from([
            Permutation {
                square: Square {
                    data: [1, 2, 3, 4, 5, 6, 7, 8, 9],
                },
            },
            Permutation {
                square: Square {
                    data: [1, 2, 3, 4, 5, 6, 7, 9, 8],
                },
            },
        ]);

        let str: String = serde_json::to_string(&b)?;
        let deser_square: BTreeSet<Permutation<O3>> = serde_json::from_str(&str)?;

        assert_eq!(str, "[[1,2,3,4,5,6,7,8,9],[1,2,3,4,5,6,7,9,8]]");
        assert_eq!(deser_square, expected);

        Ok(())
    }

    // #[test]
    // fn test_perm_iter_3() {
    //     let a = Permutation::<O3>::permutation_range(0, 8);
    //     let a_vec = a.collect::<Vec<Permutation<O3>>>();
    //     assert_eq!(8, a_vec.len());
    //     assert_eq!(7, a_vec.last().unwrap().index)
    // }

    // #[test]
    // fn test_perm_iter_4() {
    //     let a = Permutation::<O4>::permutation_range(0, 8);
    //     let a_vec = a.collect::<Vec<Permutation<O4>>>();
    //     assert_eq!(8, a_vec.len());
    //     assert_eq!(7, a_vec.last().unwrap().index)
    // }
}
