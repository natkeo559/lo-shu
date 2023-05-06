use crate::{Params, Permutation, Square, Transform};
use std::{
    collections::HashMap,
    ops::{Mul, Not},
};

#[allow(clippy::suspicious_arithmetic_impl)]
impl<P: Params + Copy> Mul for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut s = [0u8; P::ELEMENTS];
        for (i, e) in s.iter_mut().zip(rhs.square.0.into_iter()) {
            *i = self.square[(e - 1) as usize];
        }

        Square::<P>::from_array(s).to_perm()
    }
}

impl<P: Params + Copy> Not for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    type Output = Self;

    fn not(self) -> Self::Output {
        let map = self
            .square
            .0
            .into_iter()
            .zip(1..=(P::ELEMENTS as u8))
            .collect::<HashMap<_, _>>();
        let mut s = [0u8; P::ELEMENTS];
        for (i, key) in s.iter_mut().zip(1..=(P::ELEMENTS as u8)) {
            *i = *map.get(&key).unwrap();
        }

        Square::<P>::from_array(s).to_perm()
    }
}

#[cfg(test)]
mod test_op {
    use crate::{OrderThree, Permutation, Transform};

    #[test]
    fn test_op() {
        let a = Permutation::<OrderThree>::kth(227590)
            .square
            .reflect_x()
            .to_perm();

        for i in 0..50 {
            let b = Permutation::<OrderThree>::kth(i);
            println!("({}) b*a*!b = {:?}", i, (b * a * !b).cyclic_notation().k);
        }
    }
}
