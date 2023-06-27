use crate::{Params, Permutation, Square};

impl<P: Params + Copy> Square<P>
where
    [(); P::ELEMENTS]:,
{
    
    //  Time Complexity:
    //  Best = Average = Worst = O(n^2)
    //  Space Complexity:
    //  Best = Average = Worst = O(1)
    pub fn to_perm(&mut self) -> Permutation<P> {
        let mut index = 0;
        let mut position = 2;
        let mut factor = 1;
        for p in (0..=P::ELEMENTS - 2).rev() {
            let mut succ = 0;
            for q in p + 1..P::ELEMENTS {
                if self[p] > self[q] {
                    succ += 1;
                }
            }
            index += succ * factor;
            factor *= position;
            position += 1;
        }

        Permutation {
            square: *self,
            index,
        }
    }

    pub fn rotate_90(&mut self) -> Self {
        let mut a = [0; P::ELEMENTS];

        for (i, (x, y)) in a.iter_mut().zip(
            (0..P::ELEMENTS)
                .map(|x| x % P::ORDER)
                .zip((0..P::ELEMENTS).rev().map(|y| y / P::ORDER)),
        ) {
            *i = self[x * P::ORDER + y]
        }

        Square(a)
    }

    pub fn reflect_x(&mut self) -> Self {
        let mut a = [0; P::ELEMENTS];

        for (i, (x, y)) in a.iter_mut().zip(
            (0..P::ELEMENTS)
                .map(|x| x / P::ORDER)
                .zip((0..P::ELEMENTS).rev().map(|y| y % P::ORDER)),
        ) {
            *i = self[x * P::ORDER + y]
        }

        Square(a)
    }
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn rotate_90(&mut self) -> Self {
        self.square.rotate_90().to_perm()
    }

    pub fn reflect_x(&mut self) -> Self {
        self.square.reflect_x().to_perm()
    }
}

#[cfg(test)]
mod test_transform {
    use crate::{OrderFour, OrderThree, Params, Permutation, Square};

    #[test]
    fn test_id_3() {
        let mut a = Permutation::<OrderThree>::identity().square;
        assert_eq!(
            Permutation {
                square: a,
                index: 0
            },
            a.to_perm()
        );

        let mut a = Permutation::<OrderThree>::kth(OrderThree::PERMUTATIONS - 1).square;
        assert_eq!(
            Permutation {
                square: a,
                index: OrderThree::PERMUTATIONS - 1
            },
            a.to_perm()
        );

        let mut a = Permutation::<OrderThree>::kth(499).square;
        assert_eq!(
            Permutation {
                square: a,
                index: 499
            },
            a.to_perm()
        );
    }

    #[test]
    fn test_id_4() {
        let mut a = Permutation::<OrderFour>::identity().square;
        assert_eq!(
            Permutation {
                square: a,
                index: 0
            },
            a.to_perm()
        );

        let mut a = Permutation::<OrderFour>::kth(OrderFour::PERMUTATIONS - 1).square;
        assert_eq!(
            Permutation {
                square: a,
                index: OrderFour::PERMUTATIONS - 1
            },
            a.to_perm()
        );

        let mut a = Permutation::<OrderFour>::kth(499).square;
        assert_eq!(
            Permutation {
                square: a,
                index: 499
            },
            a.to_perm()
        );
    }

    #[test]
    fn test_rotate_90_3() {
        let mut a = Permutation::<OrderThree>::identity().square;
        let b = a.rotate_90();
        assert_eq!(Square([3, 6, 9, 2, 5, 8, 1, 4, 7]), b);
    }

    #[test]
    fn test_rotate_90_4() {
        let mut a = Permutation::<OrderFour>::identity().square;
        let b = a.rotate_90();
        assert_eq!(
            Square([4, 8, 12, 16, 3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13]),
            b
        );
    }

    #[test]
    fn test_reflect_x() {
        let mut a = Permutation::<OrderThree>::identity().square;
        let b = a.reflect_x();
        assert_eq!(Square([3, 2, 1, 6, 5, 4, 9, 8, 7]), b);
    }

    #[test]
    fn test_reflect_x_4() {
        let mut a = Permutation::<OrderFour>::identity().square;
        let b = a.reflect_x();
        assert_eq!(
            Square([4, 3, 2, 1, 8, 7, 6, 5, 12, 11, 10, 9, 16, 15, 14, 13]),
            b
        );
    }
}
