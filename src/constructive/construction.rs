use core::fmt;
use itertools::Itertools;
use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use crate::{ParameterSetError, Params, Permutation};

#[derive(Debug, Clone, PartialOrd)]
pub struct VecSquare<P: Params> {
    pub data: Vec<u32>,
    phantom: PhantomData<P>,
}

impl<P: Params> VecSquare<P> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: Vec::with_capacity(P::ELEMENTS),
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn fill(value: u32) -> Self {
        let data: Vec<u32> = (0..P::ELEMENTS).map(|_| value).collect();
        Self {
            data,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn identity() -> Self {
        let data = (1..=P::ELEMENTS as u32).collect();
        Self {
            data,
            phantom: PhantomData,
        }
    }

    #[must_use]
    pub fn from_vec(data: Vec<u32>) -> Self {
        Self {
            data,
            phantom: PhantomData,
        }
    }
}

impl<P: Params> Default for VecSquare<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Params> TryFrom<&[u32]> for VecSquare<P> {
    type Error = ParameterSetError;

    fn try_from(item: &[u32]) -> Result<Self, Self::Error> {
        let err =
            ParameterSetError::ElementCount("Item length does not match P::ELEMENTS!".to_string());

        if item.len() == P::ELEMENTS {
            let data: Result<Vec<u32>, std::convert::Infallible> = item.try_into();
            match data {
                Ok(data) => Ok(Self {
                    data,
                    phantom: PhantomData,
                }),
                Err(_) => Err(err),
            }
        } else {
            Err(err)
        }
    }
}

impl<P: Params> fmt::Display for VecSquare<P> {
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

impl<P: Params> Hash for VecSquare<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

impl<P: Params> PartialEq for VecSquare<P> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Construction<P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: VecSquare<P>,
}

impl<P: Params> Construction<P>
where
    [(); P::ELEMENTS]:,
{
    #[must_use]
    pub fn zeros() -> Self {
        Construction {
            square: VecSquare::from_vec(vec![0; P::ELEMENTS]),
        }
    }

    #[must_use]
    pub fn identity() -> Self {
        let square = VecSquare::from_vec((1..=P::ELEMENTS as u32).collect());
        Self { square }
    }

    /// # Errors
    ///
    /// # Panics
    ///
    #[must_use]
    pub fn siamese(seed_idx: usize) -> Self {
        assert!(P::ORDER % 2 != 0, "Order Must Be Odd!");

        let mut c = VecSquare::fill(0);

        let mut next_pos = (seed_idx / P::ORDER, seed_idx % P::ORDER);

        for i in 1..=P::ELEMENTS as u32 {
            c.data[(next_pos.0 * P::ORDER) + next_pos.1] = i;

            next_pos = (
                (next_pos.0 + (P::ORDER - 1)) % P::ORDER,
                (next_pos.1 + 1) % P::ORDER,
            );

            if c.data[next_pos.0 * P::ORDER + next_pos.1] != 0 {
                next_pos = (
                    (next_pos.0 + 2) % P::ORDER,
                    (next_pos.1 + (P::ORDER - 1)) % P::ORDER,
                );
            }
        }

        Construction { square: c }
    }
}

impl<P: Params> TryFrom<&[u32]> for Construction<P>
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
            let data = VecSquare::<P>::try_from(item);
            match data {
                Ok(data) => Ok(Self { square: data }),
                Err(_) => Err(err),
            }
        } else {
            Err(err)
        }
    }
}

impl<P: Params> TryFrom<Permutation<P>> for Construction<P>
where
    [(); P::ELEMENTS]:,
{
    type Error = ParameterSetError;

    fn try_from(item: Permutation<P>) -> Result<Self, Self::Error>
    where
        [(); P::ELEMENTS]:,
    {
        let err =
            ParameterSetError::ElementCount("Item length does not match P::ELEMENTS!".to_string());

        if item.square.len() == P::ELEMENTS {
            let data = VecSquare::<P>::try_from(item.square.data.as_slice());
            match data {
                Ok(data) => Ok(Self { square: data }),
                Err(_) => Err(err),
            }
        } else {
            Err(err)
        }
    }
}

#[cfg(test)]
mod test_construction {
    use crate::{CheckVector, Enumerable, O25, O3, O4, O5};

    use super::*;

    #[test]
    fn test_new_zeros_3() {
        let a = Construction::<O3>::zeros();
        let b = Construction {
            square: VecSquare::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_new_zeros_5() {
        let a = Construction::<O4>::zeros();
        let b = Construction {
            square: VecSquare::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_new_identity() {
        let a = Construction::<O3>::identity();
        let b = Construction {
            square: VecSquare::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_siamese_3() -> Result<(), ParameterSetError> {
        let case = Construction::<O3>::siamese(1);
        let expected = Construction {
            square: VecSquare::try_from([8, 1, 6, 3, 5, 7, 4, 9, 2].as_slice())?,
        };

        assert_eq!(case, expected);
        assert!(case.check_v().is_some());

        Ok(())
    }

    #[test]
    fn test_valid_siamese_5() -> Result<(), ParameterSetError> {
        let a = Construction::<O5>::siamese(2);
        let a_result = a.check_n_s();

        assert_eq!(Some(a), a_result);

        Ok(())
    }

    #[test]
    fn test_valid_siamese_25() -> Result<(), ParameterSetError> {
        let a = Construction::<O25>::siamese(12);
        let a_result = a.check_n_s();

        assert_eq!(Some(a), a_result);

        Ok(())
    }

    #[test]
    #[ignore = "debugging"]
    fn test_valid_siamese() -> Result<(), ParameterSetError> {
        let mut sols = 0;
        for i in 0..O5::ELEMENTS {
            let a = Construction::<O5>::siamese(i);
            if a.check_n_s().is_some() {
                let index = Permutation::try_from(a)?.index();
                sols += 1;
                println!("{}", index);
            };
        }
        println!("{}", sols);

        Ok(())
    }
}
