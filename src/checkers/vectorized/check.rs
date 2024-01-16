use crate::{Construction, Params, Permutation, Square, VecSquare, O3, O4};
use itertools::Itertools;
use std::simd::prelude::*;
use std::simd::{LaneCount, SupportedLaneCount};

pub trait CheckVector {
    type Output;

    /// Check if a square-like object is magic.
    /// Computations are vectorized and implemented using `std::simd`.
    ///
    /// # Safety
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to
    /// square-like elements. `check_v` is the safe version of this function.
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output>;

    /// Check if a square-like object is magic.
    /// Computations are vectorized and implemented using `std::simd`.
    fn check_v(&self) -> Option<Self::Output>;
}

impl CheckVector for VecSquare<O3> {
    type Output = Self;

    #[inline]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O3::MAGIC_SUM; O3::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&self.data),
            Simd::from_slice(&[*self.data.get_unchecked(8), 0, 0, 0, 0, 0, 0, 0]),
        ];

        let mut v_a = simd_swizzle!(vals[0], [0, 3, 6, 0, 1, 2, 0, 2]);
        let v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 5, 8, 6, 7, 8, 8, 6]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);

        v_a.simd_eq(VMASK).all().then_some((*self).clone())
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_v(&self) -> Option<Self::Output> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O3::MAGIC_SUM; O3::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.data.first().unwrap(),
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
        let v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 5, 8, 6, 7, 8, 8, 6]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);

        v_a.simd_eq(VMASK).all().then_some((*self).clone())
    }
}

impl CheckVector for Square<O3> {
    type Output = Self;

    #[inline]
    unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O3::MAGIC_SUM; O3::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&self.data),
            Simd::from_slice(&[*self.data.get_unchecked(8), 0, 0, 0, 0, 0, 0, 0]),
        ];

        let mut v_a = simd_swizzle!(vals[0], [0, 3, 6, 0, 1, 2, 0, 2]);
        let v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 5, 8, 6, 7, 8, 8, 6]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);

        v_a.simd_eq(VMASK).all().then_some(*self)
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_v(&self) -> Option<Self::Output> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O3::MAGIC_SUM; O3::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.data.first().unwrap(),
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
        let v_b = simd_swizzle!(vals[0], [1, 4, 7, 3, 4, 5, 4, 4]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 5, 8, 6, 7, 8, 8, 6]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);

        v_a.simd_eq(VMASK).all().then_some(*self)
    }
}

impl CheckVector for Square<O4> {
    type Output = Self;

    #[inline]
    unsafe fn check_v_unsafe(&self) -> Option<Self> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O4::MAGIC_SUM; O4::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
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

        let mut v_a = simd_swizzle!(vals[0], vals[1], [0, 0, 0, 1, 2, 3, 4, 8]);
        let v_b = simd_swizzle!(vals[0], vals[1], [1, 4, 5, 5, 6, 6, 5, 9]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 8, 10, 9, 10, 9, 6, 10]);
        let v_d = simd_swizzle!(vals[0], vals[1], [3, 12, 15, 13, 14, 12, 7, 11]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);
        v_a = v_a.saturating_add(v_d);

        v_a.simd_eq(VMASK).all().then_some(*self)
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_v(&self) -> Option<Self> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O4::MAGIC_SUM; O4::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
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

        let mut v_a = simd_swizzle!(vals[0], vals[1], [0, 0, 0, 1, 2, 3, 4, 8]);
        let v_b = simd_swizzle!(vals[0], vals[1], [1, 4, 5, 5, 6, 6, 5, 9]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 8, 10, 9, 10, 9, 6, 10]);
        let v_d = simd_swizzle!(vals[0], vals[1], [3, 12, 15, 13, 14, 12, 7, 11]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);
        v_a = v_a.saturating_add(v_d);

        v_a.simd_eq(VMASK).all().then_some(*self)
    }
}

impl CheckVector for VecSquare<O4> {
    type Output = Self;

    #[inline]
    unsafe fn check_v_unsafe(&self) -> Option<Self> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O4::MAGIC_SUM; O4::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.data.get_unchecked(0),
                *self.data.get_unchecked(1),
                *self.data.get_unchecked(2),
                *self.data.get_unchecked(3),
                *self.data.get_unchecked(4),
                *self.data.get_unchecked(5),
                *self.data.get_unchecked(6),
                *self.data.get_unchecked(7),
            ]),
            Simd::from_slice(&[
                *self.data.get_unchecked(8),
                *self.data.get_unchecked(9),
                *self.data.get_unchecked(10),
                *self.data.get_unchecked(11),
                *self.data.get_unchecked(12),
                *self.data.get_unchecked(13),
                *self.data.get_unchecked(14),
                *self.data.get_unchecked(15),
            ]),
        ];

        let mut v_a = simd_swizzle!(vals[0], vals[1], [0, 0, 0, 1, 2, 3, 4, 8]);
        let v_b = simd_swizzle!(vals[0], vals[1], [1, 4, 5, 5, 6, 6, 5, 9]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 8, 10, 9, 10, 9, 6, 10]);
        let v_d = simd_swizzle!(vals[0], vals[1], [3, 12, 15, 13, 14, 12, 7, 11]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);
        v_a = v_a.saturating_add(v_d);

        v_a.simd_eq(VMASK).all().then_some((*self).clone())
    }

    /// # Panics
    /// - If the value returned by `get` is `None`
    #[inline]
    fn check_v(&self) -> Option<Self> {
        const VMASK: Simd<u32, 8_usize> =
            Simd::from_slice(&[O4::MAGIC_SUM; O4::CONSTRAINT_VECTORS]);

        let vals: [Simd<u32, 8_usize>; 2] = [
            Simd::from_slice(&[
                *self.data.first().unwrap(),
                *self.data.get(1).unwrap(),
                *self.data.get(2).unwrap(),
                *self.data.get(3).unwrap(),
                *self.data.get(4).unwrap(),
                *self.data.get(5).unwrap(),
                *self.data.get(6).unwrap(),
                *self.data.get(7).unwrap(),
            ]),
            Simd::from_slice(&[
                *self.data.get(8).unwrap(),
                *self.data.get(9).unwrap(),
                *self.data.get(10).unwrap(),
                *self.data.get(11).unwrap(),
                *self.data.get(12).unwrap(),
                *self.data.get(13).unwrap(),
                *self.data.get(14).unwrap(),
                *self.data.get(15).unwrap(),
            ]),
        ];

        let mut v_a = simd_swizzle!(vals[0], vals[1], [0, 0, 0, 1, 2, 3, 4, 8]);
        let v_b = simd_swizzle!(vals[0], vals[1], [1, 4, 5, 5, 6, 6, 5, 9]);
        let v_c = simd_swizzle!(vals[0], vals[1], [2, 8, 10, 9, 10, 9, 6, 10]);
        let v_d = simd_swizzle!(vals[0], vals[1], [3, 12, 15, 13, 14, 12, 7, 11]);

        v_a = v_a.saturating_add(v_b);
        v_a = v_a.saturating_add(v_c);
        v_a = v_a.saturating_add(v_d);

        v_a.simd_eq(VMASK).all().then_some((*self).clone())
    }
}

// Reduce code duplication
//-------------------------------------------------------------------------------------------------

macro_rules! impl_check_vector_for_type_with_param {
    ($type:tt, $param:tt) => {
        impl CheckVector for $type<$param> {
            type Output = Self;

            #[inline(always)]
            unsafe fn check_v_unsafe(&self) -> Option<Self::Output> {
                let p = self.clone();

                if p.square.check_v_unsafe().is_some() {
                    return Some(p);
                }

                None
            }

            #[inline(always)]
            /// # Panics
            /// - If the value returned by `get` is `None`
            fn check_v(&self) -> Option<Self::Output> {
                let p = self.clone();

                if p.square.check_v().is_some() {
                    return Some(p);
                }

                None
            }
        }
    };
}

impl_check_vector_for_type_with_param!(Permutation, O3);
impl_check_vector_for_type_with_param!(Permutation, O4);
impl_check_vector_for_type_with_param!(Construction, O3);
impl_check_vector_for_type_with_param!(Construction, O4);

//-------------------------------------------------------------------------------------------------

impl<P: Params + Copy> Construction<P>
where
    [(); P::ELEMENTS]:,
{
    /// Check if a square-like object of any size is magic.
    ///
    /// `B`: represents the `LaneCount` for the SIMD arrays and can be though of the number of items
    /// taken from the contraint vectors at each step to perform the whole summation, similar to
    /// taking chunks of an iterator.
    ///
    /// # Caution
    /// This method has not been tested for all cases and/or does not output correctly for certain
    /// values of `B`. If accuracy is desired over throughput, use the `check_s_v` method.
    #[inline]
    #[must_use]
    pub fn check_n_v<const B: usize>(&self) -> Option<Self>
    where
        LaneCount<B>: SupportedLaneCount,
    {
        // Decompose the square data into two iterables.
        //
        // r: iterator over the joined rows of the square
        // c: iterator over the joined cols of the square
        let (r, c): (Vec<u32>, Vec<u32>) = (0..P::ELEMENTS)
            .map(|e| e / P::ORDER)
            .zip((0usize..P::ELEMENTS).map(|s| s % P::ORDER))
            .map(|(i, a)| {
                (
                    self.square.data[i * P::ORDER + a],
                    self.square.data[a * P::ORDER + i],
                )
            })
            .unzip();

        // Chunk over the row, col iterators so that we can iterate over whole rows and whole cols.
        let rows = r.chunks_exact(P::ORDER).collect_vec();
        let cols = c.chunks_exact(P::ORDER).collect_vec();

        // Iterators over the left and right trace
        let t1: Vec<u32> = (0..P::ORDER)
            .map(|a| self.square.data[a * (P::ORDER + 1)])
            .collect();
        let t2: Vec<u32> = (0..P::ORDER)
            .map(|a| self.square.data[(a + 1) * (P::ORDER - 1)])
            .collect();

        // Number of zeros to pad on the end of the SIMD arrays.
        let pad = if P::CONSTRAINT_VECTORS.is_power_of_two() {
            0
        } else {
            B.abs_diff(P::CONSTRAINT_VECTORS)
        };

        // Allocate the arrays used for summing.
        let mut buffs = Vec::with_capacity(P::ORDER);
        for _ in 0..P::ORDER {
            buffs.push(Vec::<u32>::with_capacity(B));
        }

        // load the data into the arrays, padding zeros at the end.
        for i in 0..P::ORDER {
            for j in rows[i] {
                buffs[i].push(*j);
            }
            for j in cols[i] {
                buffs[i].push(*j);
            }
            buffs[i].push(t1[i]);
            buffs[i].push(t2[i]);

            if pad != 0 {
                for _ in 0..pad {
                    buffs[i].push(0);
                }
            }
        }

        // The result vector, containing padding to match the summing arrays.
        let mut r_vec: Vec<u32> = vec![0; P::CONSTRAINT_VECTORS + pad];
        for item in r_vec.iter_mut().take(P::CONSTRAINT_VECTORS) {
            *item = P::MAGIC_SUM;
        }

        // Turn slices from the summing vectors into SIMD arrays and add each of them together.
        let fv = buffs
            .into_iter()
            .fold(Simd::splat(0), |a, n| a + Simd::from_slice(&n));

        // If the SIMD vector is equal to the expected result SIMD vector, then the square must be
        // magic.
        let sumv: Simd<u32, B> = Simd::from_slice(r_vec.as_slice());
        fv.simd_eq(sumv).all().then_some((*self).clone())
    }
}

#[cfg(test)]
mod test_check {
    use super::*;
    use crate::{Enumerable, ParameterSetError};

    #[test]
    fn test_safe_3() -> Result<(), ParameterSetError> {
        let a = Permutation::<O3>::kth(69074);
        let a_result = a.check_v();

        let b: Construction<O3> = Construction::try_from(a)?;
        let b_result = b.check_v();

        assert_eq!(Some(a), a_result);
        assert_eq!(Some(b), b_result);

        Ok(())
    }

    #[test]
    fn test_unsafe_3() -> Result<(), ParameterSetError> {
        let a = Permutation::<O3>::kth(69074);
        let a_result = unsafe { a.check_v_unsafe() };

        let b: Construction<O3> = Construction::try_from(a)?;
        let b_result = unsafe { b.check_v_unsafe() };

        assert_eq!(Some(a), a_result);
        assert_eq!(Some(b), b_result);

        Ok(())
    }
}
