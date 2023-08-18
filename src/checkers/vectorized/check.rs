use std::simd::{Simd, simd_swizzle, SimdUint, SimdPartialOrd, Which::{First, Second}};

use crate::{Params, O3, Square, Permutation};



pub trait CheckVector {
    type Output;

    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are vectorized and implemented using std::simd.
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output>;

    /// Checks if a Permutation of element type T, order N is magic.
    /// Computations are vectorized and implemented using std::simd.
    fn check_v(&self) -> Option<Self::Output>;
}

impl CheckVector for Square<O3> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let mut result = Self::from_array([0; O3::ELEMENTS]);
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O3::MAGIC_SUM as u32; O3::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&self.data),
            Simd::from_slice(&[*self.data.get_unchecked(8), 0, 0, 0, 0, 0, 0, 0]),
        ];

        let mut v_a = simd_swizzle!(vals[0], [0, 3, 6, 0, 1, 2, 0, 2]);
        let mut v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                First(5),
                Second(0),
                First(6),
                First(7),
                Second(0),
                Second(0),
                First(6)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        result.data = self.data;
        match v_a == VMASK {
            true => Some(result),
            false => None,
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let mut result = Self::from_array([0; O3::ELEMENTS]);
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O3::MAGIC_SUM as u32; O3::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.data.get(0).unwrap(),
                *self.data.get(1).unwrap(),
                *self.data.get(2).unwrap(),
                *self.data.get(3).unwrap(),
                *self.data.get(4).unwrap(),
                *self.data.get(5).unwrap(),
                *self.data.get(6).unwrap(),
                *self.data.get(7).unwrap(),
            ]),
            Simd::from_slice(&[*self.data.get(8).unwrap(), 0, 0, 0, 0, 0, 0, 0]),
        ];

        let mut v_a = simd_swizzle!(vals[0], [0, 3, 6, 0, 1, 2, 0, 2]);
        let mut v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                First(5),
                Second(0),
                First(6),
                First(7),
                Second(0),
                Second(0),
                First(6)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        result.data = self.data;
        match v_a == VMASK {
            true => Some(result),
            false => None,
        }
    }
}

impl CheckVector for Permutation<O3> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let mut result = Permutation::identity();
        result.square.data = self.square.data;

        if result.square.check_v_unsafe().is_some() {
            Some(result)
        } else {
            None
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let mut result = Permutation::identity();
        result.square.data = self.square.data;

        if result.square.check_v().is_some() {
            Some(result)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test_check {
    use super::*;

    #[test]
    fn test_safe_3() {
        let a = Permutation::<O3>::kth(69074);
        let a_result = a.check_v();
        assert_eq!(Some(a), a_result);
    }

    #[test]
    fn test_unsafe_3() {
        let a = Permutation::<O3>::kth(69074);
        let a_result = unsafe { a.check_v_unsafe() };
        assert_eq!(Some(a), a_result);
    }
}