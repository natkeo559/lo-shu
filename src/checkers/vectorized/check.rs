use crate::{OrderFour, OrderThree, Params, Permutation, Square};
use std::simd::{Which::*, *};

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

impl CheckVector for Square<OrderThree> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get_unchecked(0),
                *self.get_unchecked(1),
                *self.get_unchecked(2),
                *self.get_unchecked(3),
                *self.get_unchecked(4),
                *self.get_unchecked(5),
                *self.get_unchecked(6),
                *self.get_unchecked(7),
            ]),
            Simd::from_slice(&[*self.get_unchecked(8), 0, 0, 0, 0, 0, 0, 0]),
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

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get(0).unwrap(),
                *self.get(1).unwrap(),
                *self.get(2).unwrap(),
                *self.get(3).unwrap(),
                *self.get(4).unwrap(),
                *self.get(5).unwrap(),
                *self.get(6).unwrap(),
                *self.get(7).unwrap(),
            ]),
            Simd::from_slice(&[*self.get(8).unwrap(), 0, 0, 0, 0, 0, 0, 0]),
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

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl CheckVector for Square<OrderFour> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get_unchecked(0),
                *self.get_unchecked(1),
                *self.get_unchecked(2),
                *self.get_unchecked(3),
                *self.get_unchecked(4),
                *self.get_unchecked(5),
                *self.get_unchecked(6),
                *self.get_unchecked(7),
            ]),
            Simd::from_slice(&[
                *self.get_unchecked(8),
                *self.get_unchecked(9),
                *self.get_unchecked(10),
                *self.get_unchecked(11),
                *self.get_unchecked(12),
                *self.get_unchecked(13),
                *self.get_unchecked(14),
                *self.get_unchecked(15),
            ]),
        ];

        let mut v_a = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(0),
                First(0),
                First(0),
                First(1),
                First(2),
                First(3),
                First(4),
                Second(0)
            ]
        );
        let mut v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(1),
                First(4),
                First(5),
                First(5),
                First(6),
                First(6),
                First(5),
                Second(1)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                Second(0),
                Second(2),
                Second(1),
                Second(2),
                Second(1),
                First(6),
                Second(2)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(3),
                Second(4),
                Second(7),
                Second(5),
                Second(6),
                Second(4),
                First(7),
                Second(3)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self> {
        const VMASK: Simd<u8, 8_usize> =
            Simd::from_slice(&[OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

        let vals: [Simd<u8, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.get(0).unwrap(),
                *self.get(1).unwrap(),
                *self.get(2).unwrap(),
                *self.get(3).unwrap(),
                *self.get(4).unwrap(),
                *self.get(5).unwrap(),
                *self.get(6).unwrap(),
                *self.get(7).unwrap(),
            ]),
            Simd::from_slice(&[
                *self.get(8).unwrap(),
                *self.get(9).unwrap(),
                *self.get(10).unwrap(),
                *self.get(11).unwrap(),
                *self.get(12).unwrap(),
                *self.get(13).unwrap(),
                *self.get(14).unwrap(),
                *self.get(15).unwrap(),
            ]),
        ];

        let mut v_a = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(0),
                First(0),
                First(0),
                First(1),
                First(2),
                First(3),
                First(4),
                Second(0)
            ]
        );
        let mut v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(1),
                First(4),
                First(5),
                First(5),
                First(6),
                First(6),
                First(5),
                Second(1)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(2),
                Second(0),
                Second(2),
                Second(1),
                Second(2),
                Second(1),
                First(6),
                Second(2)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        v_b = simd_swizzle!(
            vals[0],
            vals[1],
            [
                First(3),
                Second(4),
                Second(7),
                Second(5),
                Second(6),
                Second(4),
                First(7),
                Second(3)
            ]
        );

        v_a = v_a.saturating_add(v_b);
        if v_a.simd_lt(v_b).any() {
            return None;
        }

        match v_a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl CheckVector for Permutation<OrderThree> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v_unsafe().is_some() {
            return Some(p);
        }

        None
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v().is_some() {
            return Some(p);
        }

        None
    }
}

impl CheckVector for Permutation<OrderFour> {
    type Output = Self;

    #[inline(always)]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v_unsafe().is_some() {
            return Some(p);
        }

        None
    }

    #[inline(always)]
    fn check_v(&self) -> Option<Self::Output> {
        let p = *self;

        if p.square.check_v().is_some() {
            return Some(p);
        }

        None
    }
}

#[cfg(test)]
mod check_tests {
    use super::*;

    #[test]
    fn check_unchecked_single() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = unsafe { a.check_v_unsafe() };
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
    }

    #[test]
    fn check_single() {
        let a = Permutation::<OrderThree>::kth(69074);
        let r = a.check_v();
        assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
    }
}
