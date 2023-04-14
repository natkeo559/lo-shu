use crate::{OrderFour, OrderThree, Params, Permutation};
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

macro_rules! impl_checkers {
    ($t:ty, $p:tt) => {
        impl Check<$p> for Permutation<$p> {
            unsafe fn check_unsafe(&self) -> Option<Permutation<$p>> {
                static VMASK: Simd<$t, 8_usize> =
                    Simd::from_array([$p::MAGIC_SUM as $t; $p::CONSTRAINT_VECTORS]);

                let mut a: Simd<$t, 8_usize> = Simd::from_array([
                    *self.square.get_unchecked(0),
                    *self.square.get_unchecked(3),
                    *self.square.get_unchecked(6),
                    *self.square.get_unchecked(0),
                    *self.square.get_unchecked(1),
                    *self.square.get_unchecked(2),
                    *self.square.get_unchecked(0),
                    *self.square.get_unchecked(2),
                ]);

                let mut b: Simd<$t, 8_usize> = Simd::from_array([
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

            fn check(&self) -> Option<Permutation<$p>> {
                static VMASK: Simd<$t, 8_usize> =
                    Simd::from_array([$p::MAGIC_SUM as $t; $p::CONSTRAINT_VECTORS]);

                let mut a: Simd<$t, 8_usize> = Simd::from_array([
                    *self.square.get(0).unwrap(),
                    *self.square.get(3).unwrap(),
                    *self.square.get(6).unwrap(),
                    *self.square.get(0).unwrap(),
                    *self.square.get(1).unwrap(),
                    *self.square.get(2).unwrap(),
                    *self.square.get(0).unwrap(),
                    *self.square.get(2).unwrap(),
                ]);

                let mut b: Simd<$t, 8_usize> = Simd::from_array([
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
    };
}

impl Check<OrderFour> for Permutation<OrderFour> {
    unsafe fn check_unsafe(&self) -> Option<Permutation<OrderFour>> {
        static VMASK: Simd<u8, 8_usize> =
            Simd::from_array([OrderFour::MAGIC_SUM as u8; OrderFour::CONSTRAINT_VECTORS - 2]);

        // Ordered data deps -- Cache
        // 0  1  2  3
        // 0  4  8  12
        // 0  5  10 15
        // 1  5  9  13
        // 2  6  10 14
        // 3  6  9  12
        // 4  5  6  7
        // 8  9  10 11

        let mut a: Simd<u8, 8_usize> = Simd::from_array([
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(8),
        ]);

        let mut b: Simd<u8, 8_usize> = Simd::from_array([
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(9),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        b = Simd::from_array([
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(10),
            *self.square.get_unchecked(9),
            *self.square.get_unchecked(10),
            *self.square.get_unchecked(9),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(10),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        b = Simd::from_array([
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(12),
            *self.square.get_unchecked(15),
            *self.square.get_unchecked(13),
            *self.square.get_unchecked(14),
            *self.square.get_unchecked(12),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(11),
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

    fn check(&self) -> Option<Permutation<OrderFour>> {
        static VMASK: Simd<u8, 8_usize> =
            Simd::from_array([OrderFour::MAGIC_SUM as u8; OrderFour::CONSTRAINT_VECTORS - 2]);

        let mut a: Simd<u8, 8_usize> = Simd::from_array([
            *self.square.get(0).unwrap(),
            *self.square.get(0).unwrap(),
            *self.square.get(0).unwrap(),
            *self.square.get(1).unwrap(),
            *self.square.get(2).unwrap(),
            *self.square.get(3).unwrap(),
            *self.square.get(4).unwrap(),
            *self.square.get(8).unwrap(),
        ]);

        let mut b: Simd<u8, 8_usize> = Simd::from_array([
            *self.square.get(1).unwrap(),
            *self.square.get(4).unwrap(),
            *self.square.get(5).unwrap(),
            *self.square.get(5).unwrap(),
            *self.square.get(6).unwrap(),
            *self.square.get(6).unwrap(),
            *self.square.get(5).unwrap(),
            *self.square.get(9).unwrap(),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        b = Simd::from_array([
            *self.square.get(2).unwrap(),
            *self.square.get(8).unwrap(),
            *self.square.get(10).unwrap(),
            *self.square.get(9).unwrap(),
            *self.square.get(10).unwrap(),
            *self.square.get(9).unwrap(),
            *self.square.get(6).unwrap(),
            *self.square.get(10).unwrap(),
        ]);

        a = a.saturating_add(b);
        if a.simd_lt(b).any() {
            return None;
        }

        b = Simd::from_array([
            *self.square.get(3).unwrap(),
            *self.square.get(12).unwrap(),
            *self.square.get(15).unwrap(),
            *self.square.get(13).unwrap(),
            *self.square.get(14).unwrap(),
            *self.square.get(12).unwrap(),
            *self.square.get(7).unwrap(),
            *self.square.get(11).unwrap(),
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

impl_checkers!(u8, OrderThree);

// impl_checkers_four!(u8, OrderFour);

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
