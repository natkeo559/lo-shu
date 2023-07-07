use crate::{Params, Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Construction<P: Params + Copy>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<P>,
}

impl<P: Params + Copy> Construction<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn zeros() -> Self {
        Construction {
            square: Square([0; P::ELEMENTS])
        }
    }

    pub fn identity() -> Self {
        let mut arr: [u8; P::ELEMENTS] = [0; P::ELEMENTS];
        for (elem, val) in arr.iter_mut().zip(1..=P::ELEMENTS as u8) {
            *elem = val;
        }

        Self {
            square: Square(arr)
        }

    }

    pub fn siamese(seed_idx: usize) -> Self {
        assert!(P::ORDER % 2 != 0, "Order Must Be Odd!");

        let mut c = Self::zeros();

        let mut next_pos = (seed_idx / P::ORDER, seed_idx % P::ORDER);

        for i in 1..=P::ELEMENTS as u8 {

            c.square[(next_pos.0 * P::ORDER) + next_pos.1] = i;

            next_pos = ((next_pos.0 + (P::ORDER - 1)) % P::ORDER, (next_pos.1 + 1) % P::ORDER);

            if c.square[next_pos.0 * P::ORDER + next_pos.1] != 0 {
                next_pos = ((next_pos.0 + 2) % P::ORDER, (next_pos.1 + (P::ORDER - 1)) % P::ORDER);
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
        Self::zeros()
    }
}

#[cfg(test)]
mod test_construction {
    use crate::{CheckVector, OrderThree, OrderFive};

    use super::*;

    #[test]
    fn test_new_zeros() {
        let a = Construction::<OrderThree>::zeros();
        let b = Construction {
            square: Square([0, 0, 0, 0, 0, 0, 0, 0, 0]),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_new_identity() {
        let a = Construction::<OrderThree>::identity();
        let b = Construction {
            square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9]),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn test_siamese() {
        let a = Construction::<OrderThree>::siamese(1);
        let b = Construction {
            square: Square([8, 1, 6, 3, 5, 7, 4, 9, 2]),
        };

        assert_eq!(a, b);
        assert!(a.check_v().is_some())
    }

    #[test]
    #[ignore = "debugging"]
    fn test_valid_siamese() {
        for i in 0..OrderThree::ELEMENTS {
            if let Some(a) = Construction::<OrderThree>::siamese(i).check_v() {
                println!("{}", a.square)
            };
        }

        let mut sols = vec![];
        for i in 0..OrderFive::ELEMENTS {
            let a = Construction::<OrderFive>::siamese(i);
            if a.check_n_v().is_some() {
                sols.push(a);
                println!("{}\n", a.square)
            };
        }
        println!("{}", sols.len())
    }
}
