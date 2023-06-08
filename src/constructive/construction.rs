use crate::{Params, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Construction<P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<P>,
}

impl<P: Params + Copy> Construction<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn new() -> Self {
        Construction {
            square: Square::<P>::from_array([0; P::ELEMENTS]),
        }
    }

    pub fn siamese(&self, seed_idx: usize) -> Self {
        assert!(P::ORDER % 2 != 0, "Order Must Be Odd!");

        let mut c = *self;

        let mut next_pos = (seed_idx / 3, seed_idx % 3);
        for i in 1..=P::ELEMENTS as u8 {
            c.square[(next_pos.0 * P::ORDER) + next_pos.1] = i;
            next_pos = ((next_pos.0 + 2) % P::ORDER, (next_pos.1 + 1) % P::ORDER);
            if c.square[(next_pos.0 * P::ORDER) + next_pos.1] != 0 {
                next_pos = ((next_pos.0 + 2) % P::ORDER, (next_pos.1 + 2) % P::ORDER);
            }
        }

        c
    }
}

impl<P: Params + Copy> Default for Construction<P>
where
    [(); P::ELEMENTS]:,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test_construction {
    use crate::{CheckVector, OrderThree};

    use super::*;

    #[test]
    fn test_new() {
        let a = Construction::<OrderThree>::new();
        let b = Construction {
            square: Square([0, 0, 0, 0, 0, 0, 0, 0, 0]),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_siamese() {
        let a = Construction::<OrderThree>::new();
        let b = a.siamese(1);
        println!("{}", b.square)
    }

    #[test]
    fn test_valid_siamese() {
        for i in 0..OrderThree::ELEMENTS {
            if let Some(a) = Construction::<OrderThree>::new().siamese(i).check_v() {
                println!("{}", a.square)
            };
        }
    }
}
