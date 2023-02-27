use crate::{Permutation, Square};

pub trait SquareTransforms<T: Copy + Clone, const N: usize> {
    fn perm_id(&mut self) -> Permutation<T, N>;
    fn rotate_90(&mut self) -> Self;
    fn reflect_x(&mut self) -> Self;
}

impl SquareTransforms<u8, 9> for Square<u8, 9> {
    fn perm_id(&mut self) -> Permutation<u8, 9> {
        let n = 9;

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
        Square([
            self[6], self[3], self[0], self[7], self[4], self[1], self[8], self[5], self[2],
        ])
    }

    fn reflect_x(&mut self) -> Self {
        Square([
            self[2], self[1], self[0], self[5], self[4], self[3], self[8], self[7], self[6],
        ])
    }
}

#[cfg(test)]
mod test_transform {
    use crate::{OrderThree, Params, Permutation, Square, SquareTransforms};

    #[test]
    fn test_id() {
        let mut a = Permutation::<u8, 9>::first().square;
        assert_eq!(
            Permutation {
                square: a,
                index: 0
            },
            a.perm_id()
        );

        let mut a = Permutation::<u8, 9>::kth(OrderThree::PERMUTATIONS - 1).square;
        assert_eq!(
            Permutation {
                square: a,
                index: OrderThree::PERMUTATIONS - 1
            },
            a.perm_id()
        );

        let mut a = Permutation::<u8, 9>::kth(499).square;
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
        let mut a = Permutation::<u8, 9>::first().square;
        let b = a.rotate_90().rotate_90();
        assert_eq!(Square([9, 8, 7, 6, 5, 4, 3, 2, 1]), b);
    }

    #[test]
    fn test_reflect_x() {
        let mut a = Permutation::<u8, 9>::first().square;
        let b = a.reflect_x();
        assert_eq!(Square([3, 2, 1, 6, 5, 4, 9, 8, 7]), b);
    }
}
