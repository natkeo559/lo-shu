use crate::{Square, Permutation};

pub trait SquareTransforms<T: Copy + Clone, const N: usize> {
    fn perm_id(&mut self) -> Permutation<T, N>;
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
        Permutation { square: *self, index: result - 46234}
        
    }
}

#[cfg(test)]
mod test_transform {
    use crate::{OrderThree, Params, Permutation, SquareTransforms};

    #[test]
    fn test_id() {
        let mut a = Permutation::<u8, 9>::first().square;
        assert_eq!(Permutation { square: a, index: 0}, a.perm_id());

        let mut a = Permutation::<u8, 9>::kth(OrderThree::PERMUTATIONS - 1).square;
        assert_eq!(Permutation { square: a, index: OrderThree::PERMUTATIONS - 1}, a.perm_id());

        let mut a = Permutation::<u8, 9>::kth(499).square;
        assert_eq!(Permutation { square: a, index: 499}, a.perm_id());
    }
}
