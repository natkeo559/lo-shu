use crate::{Params, Permutation, Square};

impl<P: Params + Copy> Square<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn rotate_90(&mut self) -> Self {
        let mut data = [0; P::ELEMENTS];

        for (i, (x, y)) in data.iter_mut().zip(
            (0..P::ELEMENTS)
                .map(|x| x % P::ORDER)
                .zip((0..P::ELEMENTS).rev().map(|y| y / P::ORDER)),
        ) {
            *i = self[x * P::ORDER + y]
        }

        Square { data }
    }

    pub fn reflect_x(&mut self) -> Self {
        let mut data = [0; P::ELEMENTS];

        for (i, (x, y)) in data.iter_mut().zip(
            (0..P::ELEMENTS)
                .map(|x| x / P::ORDER)
                .zip((0..P::ELEMENTS).rev().map(|y| y % P::ORDER)),
        ) {
            *i = self[x * P::ORDER + y]
        }

        Square { data }
    }

    pub fn to_perm(&self) -> Permutation<P> {
        Permutation { square: *self }
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
    use crate::{O4, O3, Permutation, Square};

    #[test]
    fn test_rotate_90_3() {
        let mut a = Permutation::<O3>::identity().square;
        let b = a.rotate_90();
        assert_eq!(Square { data: [3, 6, 9, 2, 5, 8, 1, 4, 7] }, b);
    }

    #[test]
    fn test_rotate_90_4() {
        let mut a = Permutation::<O4>::identity().square;
        let b = a.rotate_90();
        assert_eq!(
            Square { data: [4, 8, 12, 16, 3, 7, 11, 15, 2, 6, 10, 14, 1, 5, 9, 13] } ,
            b
        );
    }

    #[test]
    fn test_reflect_x() {
        let mut a = Permutation::<O3>::identity().square;
        let b = a.reflect_x();
        assert_eq!(Square { data: [3, 2, 1, 6, 5, 4, 9, 8, 7] }, b);
    }

    #[test]
    fn test_reflect_x_4() {
        let mut a = Permutation::<O4>::identity().square;
        let b = a.reflect_x();
        assert_eq!(
            Square { data: [4, 3, 2, 1, 8, 7, 6, 5, 12, 11, 10, 9, 16, 15, 14, 13] },
            b
        );
    }
}
