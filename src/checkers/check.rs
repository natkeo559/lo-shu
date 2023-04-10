use crate::{OrderThree, Params, Permutation};
use std::simd::*;

pub trait Check<P: Params>
where
    [(); P::ELEMENTS]:,
{
    /// Checks if a Permutation of element type T, order N is magic.
    ///
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    unsafe fn check_unsafe(&self) -> Option<Permutation<P>>;

    /// Checks if a Permutation of element type T, order N is magic.
    fn check(&self) -> Option<Permutation<P>>;
}

impl Check<OrderThree> for Permutation<OrderThree> {
    unsafe fn check_unsafe(&self) -> Option<Permutation<OrderThree>> {
        static VMASK: Simd<u8, 8_usize> =
            Simd::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let mut a: Simd<u8, 8_usize> = u8x8::from_array([
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(2),
        ]);

        let mut b: Simd<u8, 8_usize> = u8x8::from_array([
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(4),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        b = Simd::from_array([
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }

    fn check(&self) -> Option<Permutation<OrderThree>> {
        static VMASK: Simd<u8, 8_usize> =
            u8x8::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let mut a: Simd<u8, 8_usize> = Simd::from_array([
            *self.square.get(0).unwrap(),
            *self.square.get(3).unwrap(),
            *self.square.get(6).unwrap(),
            *self.square.get(0).unwrap(),
            *self.square.get(1).unwrap(),
            *self.square.get(2).unwrap(),
            *self.square.get(0).unwrap(),
            *self.square.get(2).unwrap(),
        ]);

        let mut b: Simd<u8, 8_usize> = Simd::from_array([
            *self.square.get(1).unwrap(),
            *self.square.get(4).unwrap(),
            *self.square.get(7).unwrap(),
            *self.square.get(3).unwrap(),
            *self.square.get(4).unwrap(),
            *self.square.get(5).unwrap(),
            *self.square.get(4).unwrap(),
            *self.square.get(4).unwrap(),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        b = Simd::from_array([
            *self.square.get(2).unwrap(),
            *self.square.get(5).unwrap(),
            *self.square.get(8).unwrap(),
            *self.square.get(6).unwrap(),
            *self.square.get(7).unwrap(),
            *self.square.get(8).unwrap(),
            *self.square.get(8).unwrap(),
            *self.square.get(6).unwrap(),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

#[cfg(test)]
mod check_tests {
    use super::*;

    #[test]
    fn check_unchecked_single() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = unsafe { a.check_unsafe() };
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
    }

    #[test]
    fn check_single() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = a.check();
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
    }
}