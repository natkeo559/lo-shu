use crate::{
    unpack_u4x16, unpack_u4x2, unpack_u4x4, unpack_u4x8, CheckVector, CompressedPermutation,
    OrderThree, Params,
};
use std::simd::*;

macro_rules! impl_compressed_checkers {
    ($t:ty, $l:literal, $func:ident) => {
        impl CheckVector for CompressedPermutation<$t, $l, OrderThree> {
            type Output = usize;
            /// Checks if Permutation of element type T, order N is magic.
            ///
            /// # Note
            ///
            /// Checking a CompressedPermutation containing multiple valid Permutations may yield incorrect results. it is assumed that a CompressedPermutaion only contains one valid Permutaion.
            ///
            /// # Safety
            ///
            /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
            unsafe fn check_v_unsafe(&self) -> Option<usize> {
                const VMASK: Simd<u8, 8_usize> =
                    Simd::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);
                const MASK: Simd<$t, 8_usize> =
                    Simd::from_array([OrderThree::MAGIC_SUM as $t; OrderThree::CONSTRAINT_VECTORS]);

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

                let add_b = a + b;

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

                // 4-bit saturating add all lanes
                a = (0..$l as $t).fold(Simd::splat(0), |n, i| {
                    n | ((add_b & MASK << Simd::splat(i * 4)) + b) & MASK << Simd::splat(i * 4)
                });

                // decompress
                let z = &a.as_array().map($func);
                let mut t = [[0u8; OrderThree::CONSTRAINT_VECTORS]; $l];

                // Transpose z
                for (i, t_item) in t.iter_mut().enumerate() {
                    for (j, z_item) in z.iter().enumerate() {
                        t_item[j] = z_item[i]
                    }
                }

                // Find index of valid magic square if one exists
                for (idx, i) in t.iter().enumerate() {
                    if Simd::from_slice(i) == VMASK {
                        return Some(self.index[idx]);
                    }
                }
                None
            }

            /// Checks if Permutation of element type T, order N is magic.
            fn check_v(&self) -> Option<usize> {
                const VMASK: Simd<u8, 8_usize> =
                    Simd::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);
                static MASK: Simd<$t, 8_usize> =
                    Simd::from_array([OrderThree::MAGIC_SUM as $t; OrderThree::CONSTRAINT_VECTORS]);

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

                let add_b = a + b;

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

                // 4-bit saturating add all lanes
                a = (0..$l as $t).fold(Simd::splat(0), |n, i| {
                    n | ((add_b & MASK << Simd::splat(i * 4)) + b) & MASK << Simd::splat(i * 4)
                });

                // decompress
                let z = &a.as_array().map($func);
                let mut t = [[0u8; OrderThree::CONSTRAINT_VECTORS]; $l];

                // Transpose z
                for (i, t_item) in t.iter_mut().enumerate() {
                    for (j, z_item) in z.iter().enumerate() {
                        t_item[j] = z_item[i]
                    }
                }

                // Find index of valid magic square if one exists
                for (idx, i) in t.iter().enumerate() {
                    if u8x8::from_slice(i) == VMASK {
                        return Some(self.index[idx]);
                    }
                }
                None
            }
        }
    };
}

impl_compressed_checkers!(u8, 2, unpack_u4x2);
impl_compressed_checkers!(u16, 4, unpack_u4x4);
impl_compressed_checkers!(u32, 8, unpack_u4x8);
impl_compressed_checkers!(u64, 16, unpack_u4x16);

#[cfg(test)]
mod test_check {
    use super::*;
    use crate::Permutation;

    #[test]
    fn check_compressed_2x() {
        let p = Permutation::<OrderThree>::permutation_range(69073, 69075);
        let b = CompressedPermutation::<u8, 2, OrderThree>::compress_two_from_p_iter(p);
        let c = unsafe { b.check_v_unsafe() };
        assert_eq!(Some(69074), c)
    }

    #[test]
    fn check_compressed_4x() {
        let p = Permutation::<OrderThree>::permutation_range(69073, 69077);
        let b = CompressedPermutation::<u16, 4, OrderThree>::compress_four_from_p_iter(p);
        let c = unsafe { b.check_v_unsafe() };
        assert_eq!(Some(69074), c)
    }

    #[test]
    fn check_compressed_8x() {
        let p = Permutation::<OrderThree>::permutation_range(69073, 69081);
        let b = CompressedPermutation::<u32, 8, OrderThree>::compress_eight_from_p_iter(p);
        let c = unsafe { b.check_v_unsafe() };
        assert_eq!(Some(69074), c)
    }

    #[test]
    fn check_compressed_16x() {
        let p = Permutation::<OrderThree>::permutation_range(69073, 69089);
        let b = CompressedPermutation::<u64, 16, OrderThree>::compress_sixteen_from_p_iter(p);
        let c = unsafe { b.check_v_unsafe() };
        assert_eq!(Some(69074), c)
    }
}
