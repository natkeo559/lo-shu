use std::{marker::PhantomData, hash::{Hash, Hasher}};
use core::fmt;
use itertools::Itertools;

use crate::Params;

#[derive(Debug, Clone, PartialOrd)]
pub struct VecSquare<P: Params> {
    pub data: Vec<u32>,
    phantom: PhantomData<P>
}

impl<P: Params> VecSquare<P> {
    pub fn new() -> Self {
        Self { data: Vec::with_capacity(P::ELEMENTS), phantom: PhantomData }
    }

    pub fn fill(value: u32) -> Self {
        let data: Vec<u32> = (0..P::ELEMENTS).map(|_| value).collect();
        Self { data, phantom: PhantomData }
    }

    pub fn identity() -> Self {
        let data = (1..=P::ELEMENTS as u32).collect();
        Self { data, phantom: PhantomData }
    }

    pub fn from_vec(data: Vec<u32>) -> Self{
        Self { data, phantom: PhantomData}
    }
}

impl<P: Params> Default for VecSquare<P> {
    fn default() -> Self {
        Self::new()
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
        self.data.hash(state)
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
    pub fn zeros() -> Self {
        Construction {
            square: VecSquare::from_vec(vec![0; P::ELEMENTS]),
        }
    }

    pub fn identity() -> Self {
        let square = VecSquare::from_vec((1..=P::ELEMENTS as u32).collect());
        Self { square }
    }

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

#[cfg(test)]
mod test_construction {
    use crate::{CheckVector, O3, O5, O25};

    use super::*;

    #[test]
    fn test_new_zeros() {
        let a = Construction::<O3>::zeros();
        let b = Construction {
            square: VecSquare::from_vec(vec![0, 0, 0, 0, 0, 0, 0, 0, 0])
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_new_identity() {
        let a = Construction::<O3>::identity();
        let b = Construction {
            square: VecSquare::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9])
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_siamese() {
        let a = Construction::<O3>::siamese(1);
        let b = Construction {
            square: VecSquare::from_vec(vec![8, 1, 6, 3, 5, 7, 4, 9, 2])
        };

        assert_eq!(a, b);
        assert!(a.check_v().is_some())
    }

    #[test]
    fn test_siamese2() {
        let a = Construction::<O3>::siamese(1);
        let b = Construction {
            square: VecSquare {
                data: vec![8, 1, 6, 3, 5, 7, 4, 9, 2],
                phantom: PhantomData
            },
        };

        assert_eq!(a, b);
        assert!(a.check_v().is_some())
    }

    #[test]
    #[ignore = "debugging"]
    fn test_valid_siamese() {
        for i in 0..O3::ELEMENTS {
            if let Some(a) = Construction::<O3>::siamese(i).check_v() {
                println!("{}", a.square)
            };
        }
        let mut sols = vec![];
        for i in 0..O25::ELEMENTS {
            let a = Construction::<O25>::siamese(i);
            if a.check_n_v::<32>().is_some() {
                sols.push(a.clone());
                // println!("{}", i);
                // println!("{}\n", a.square)
            };
        }
        println!("\n\n\n\n\n{}\n\n\n\n", sols.len())
    }
}
