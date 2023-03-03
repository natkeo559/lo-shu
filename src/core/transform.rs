use crate::{Params, Permutation, Square};

pub trait SquareTransforms<T: Copy + Clone, P: Params>
where
    [(); P::ELEMENTS]:,
{
    fn perm_id(&mut self) -> Permutation<T, P>;
    fn rotate_90(&mut self) -> Self;
    // fn reflect_x(&mut self) -> Self;
}

impl<P: Params + Copy> SquareTransforms<u8, P> for Square<u8, P>
where
    [(); P::ELEMENTS]:,
{
    fn perm_id(&mut self) -> Permutation<u8, P> {
        let n = P::ELEMENTS;

        let mut result = 0usize;
        let mut j = 0;
        while j < n {
            let mut i = 1;
            let mut factor = 1;
            while i < n - j {
                factor *= i;
                i += 1;
            }
            i = 0;
            let mut index = self[j] as usize;
            while i < j {
                if self[i] < self[j] {
                    index -= 1;
                }
                i += 1;
            }
            result += index * factor;
            j += 1;
        }
        Permutation {
            square: *self,
            index: result - 46234,
        }
    }

    fn rotate_90(&mut self) -> Self {
        let mut a = [0; P::ELEMENTS];

        for (i, (x, y)) in (0..P::ELEMENTS)
            .map(|x| x % P::ORDER)
            .zip((0..P::ELEMENTS).map(|y| y / P::ORDER).rev())
            .enumerate()
        {
            a[i] = self[x * P::ORDER + y]
        }

        Square(a)
    }

    // fn reflect_x(&mut self) -> Self {
    //     Square([
    //         self[2], self[1], self[0], self[5], self[4], self[3], self[8], self[7], self[6],
    //     ])
    // }
}

#[cfg(test)]
mod test_transform {
    use crate::{OrderThree, Params, Permutation, Square, SquareTransforms};

    #[test]
    fn test_id() {
        let mut a = Permutation::<u8, OrderThree>::first().square;
        assert_eq!(
            Permutation {
                square: a,
                index: 0
            },
            a.perm_id()
        );

        let mut a = Permutation::<u8, OrderThree>::kth(OrderThree::PERMUTATIONS - 1).square;
        assert_eq!(
            Permutation {
                square: a,
                index: OrderThree::PERMUTATIONS - 1
            },
            a.perm_id()
        );

        let mut a = Permutation::<u8, OrderThree>::kth(499).square;
        assert_eq!(
            Permutation {
                square: a,
                index: 499
            },
            a.perm_id()
        );
    }

    #[test]
    fn test_rotate_90() {
        let mut a = Permutation::<u8, OrderThree>::first().square;
        let b = a.rotate_90().rotate_90();
        assert_eq!(Square([9, 8, 7, 6, 5, 4, 3, 2, 1]), b);
    }

    // #[test]
    // fn test_reflect_x() {
    //     let mut a = Permutation::<u8, OrderThree>::first().square;
    //     let b = a.reflect_x();
    //     assert_eq!(Square([3, 2, 1, 6, 5, 4, 9, 8, 7]), b);
    // }
}
