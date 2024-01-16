use crate::{Params, Permutation, Square};
use std::collections::HashMap;
use std::ops::{Mul, MulAssign};

#[allow(clippy::suspicious_arithmetic_impl)]
impl<P: Params + Copy> Mul for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut set = [0; P::ELEMENTS];
        for (set_x, rhs_x) in set
            .iter_mut()
            .zip(rhs.square.data.into_iter().map(|x| x - 1))
        {
            *set_x = self.square[rhs_x as usize];
        }

        Square::<P>::from_array(set).to_perm()
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl<P: Params + Copy> MulAssign for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    /// Computes the inverse of the permutation.
    /// # Panics
    /// - If key to corresponding map does not exist.
    #[must_use]
    pub fn inv(&self) -> Self {
        let map = self
            .square
            .data
            .into_iter()
            .zip(1..=(P::ELEMENTS as u32))
            .collect::<HashMap<_, _>>();
        let mut s = [0u32; P::ELEMENTS];
        for (i, key) in s.iter_mut().zip(1..=(P::ELEMENTS as u32)) {
            *i = *map.get(&key).unwrap();
        }

        Square::<P>::from_array(s).to_perm()
    }
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    /// Raises the permutation to the power of `n`.
    #[must_use]
    pub fn pow(&self, n: i32) -> Self {
        if n == 0 {
            return Permutation::identity();
        }
        let mut init = *self;
        if n.is_negative() {
            init = init.inv();
        }
        (1..n.abs()).map(|_| init).fold(init, |a, b| b * a)
    }
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    /// Conjugates the permutation by another permutation.
    #[must_use]
    pub fn conj(&self, conjugate: Self) -> Self {
        conjugate.inv() * *self * conjugate
    }
}

#[cfg(test)]
mod test_ops {
    use crate::{Enumerable, Permutation, O3};

    #[test]
    fn test_mul() {
        let e = Permutation::<O3>::identity();
        let mut a = Permutation::<O3>::kth(400);
        let b = e.clone().rotate_90();
        let r = a.clone().rotate_90();

        let c = a * b;
        assert_eq!(r, c);

        a *= b;
        assert_eq!(c, a);
        assert_eq!(b, e * b)
    }

    #[test]
    fn test_pow() {
        let e = Permutation::<O3>::identity();
        let a = e.clone().rotate_90();

        assert_eq!(a.inv() * a.inv(), a.pow(-2));
        assert_eq!(a.inv(), a.pow(-1));
        assert_eq!(e, a.pow(0));
        assert_eq!(a, a.pow(1));
        assert_eq!(a * a, a.pow(2));
    }

    #[test]
    fn test_inv() {
        let e = Permutation::<O3>::identity();
        let b = e.clone().rotate_90();
        let a = Permutation::<O3>::kth(400);

        assert_eq!(e, a * a.inv());
        assert_eq!(a, a * b * b.inv());
        assert_eq!(a.inv() * b, (b.inv() * a).inv());
        assert_eq!(a * b, (b.inv() * a.inv()).inv());
        assert_eq!(b.inv() * a.inv(), (a * b).inv());
    }

    #[test]
    fn test_conj() {
        let a = Permutation::<O3>::identity().rotate_90();
        let b = Permutation::<O3>::identity().reflect_x();
        println!("{}", b.conj(a));
        println!("{}", a.conj(b));
    }
}
