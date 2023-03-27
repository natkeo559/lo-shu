use crate::{
    params::Params, unpack_u4x16, unpack_u4x2, unpack_u4x4, unpack_u4x8, CompressedPermutation,
    OrderThree, Permutation,
};
use std::simd::*;

pub trait Check<T: Clone + Copy, P: Params>
where
    [(); P::ELEMENTS]:,
{
    /// Checks if a Permutation of element type T, order N is magic.
    ///
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    unsafe fn unsafe_check_strict(&mut self) -> Option<Permutation<P>>;
}

pub trait CheckCompressed<P: Params>
where
    [(); P::ELEMENTS]:,
{
    /// Checks if Permutation of element type T, order N is magic.
    ///
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    unsafe fn unsafe_check_strict(&mut self) -> Vec<Option<usize>>;
}

impl Check<u8, OrderThree> for Permutation<OrderThree> {
    unsafe fn unsafe_check_strict(&mut self) -> Option<Permutation<OrderThree>> {
        static VMASK: Simd<u8, 8_usize> =
            u8x8::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);

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

        a += &b;

        b = u8x8::from_array([
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
        ]);

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl CheckCompressed<OrderThree> for CompressedPermutation<u8, 2, OrderThree> {
    unsafe fn unsafe_check_strict(&mut self) -> Vec<Option<usize>> {
        static VMASK: Simd<u8, 8_usize> =
            u8x8::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);
        static MASK0: Simd<u8, 8_usize> = u8x8::from_array([0x0F; OrderThree::CONSTRAINT_VECTORS]);
        static MASK1: Simd<u8, 8_usize> = u8x8::from_array([0xF0; OrderThree::CONSTRAINT_VECTORS]);

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

        let b: Simd<u8, 8_usize> = u8x8::from_array([
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(4),
        ]);

        let c: Simd<u8, 8_usize> = u8x8::from_array([
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
        ]);

        let add_b = a + b;
        a = ((add_b & MASK0) + c) & MASK0 | ((add_b & MASK1) + c) & MASK1;

        let a_arr = *a.as_array();

        let mut unpacked = [[0u8; OrderThree::CONSTRAINT_VECTORS]; 2];
        for (index, packed_elem) in a_arr.into_iter().enumerate() {
            let a = unpack_u4x2(packed_elem);
            for i in 0..2 {
                unpacked[i][index] = a[i];
            }
        }

        let mut result = vec![];
        for (index, item) in unpacked.into_iter().enumerate() {
            if u8x8::from_array(item) == VMASK {
                result.push(Some(self.index[index]));
            } else {
                result.push(None);
            }
        }
        result
    }
}

impl CheckCompressed<OrderThree> for CompressedPermutation<u16, 4, OrderThree> {
    unsafe fn unsafe_check_strict(&mut self) -> Vec<Option<usize>> {
        static VMASK: Simd<u8, 8_usize> =
            u8x8::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);
        static MASK0: Simd<u16, 8_usize> =
            u16x8::from_array([0x00_00_00_0F; OrderThree::CONSTRAINT_VECTORS]);
        static MASK1: Simd<u16, 8_usize> =
            u16x8::from_array([0x00_00_00_F0; OrderThree::CONSTRAINT_VECTORS]);
        static MASK2: Simd<u16, 8_usize> =
            u16x8::from_array([0x00_00_0F_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK3: Simd<u16, 8_usize> =
            u16x8::from_array([0x00_00_F0_00; OrderThree::CONSTRAINT_VECTORS]);

        let mut a: Simd<u16, 8_usize> = u16x8::from_array([
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(2),
        ]);

        let b: Simd<u16, 8_usize> = u16x8::from_array([
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(4),
        ]);

        let c: Simd<u16, 8_usize> = u16x8::from_array([
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
        ]);

        let add_b = a + b;
        a = ((add_b & MASK0) + c) & MASK0
            | ((add_b & MASK1) + c) & MASK1
            | ((add_b & MASK2) + c) & MASK2
            | ((add_b & MASK3) + c) & MASK3;

        let a_arr = *a.as_array();

        let mut unpacked = [[0u8; OrderThree::CONSTRAINT_VECTORS]; 4];
        for (index, packed_elem) in a_arr.into_iter().enumerate() {
            let a = unpack_u4x4(packed_elem);
            for i in 0..4 {
                unpacked[i][index] = a[i];
            }
        }

        let mut result = vec![];
        for (index, item) in unpacked.into_iter().enumerate() {
            if u8x8::from_array(item) == VMASK {
                result.push(Some(self.index[index]));
            } else {
                result.push(None);
            }
        }
        result
    }
}

impl CheckCompressed<OrderThree> for CompressedPermutation<u32, 8, OrderThree> {
    unsafe fn unsafe_check_strict(&mut self) -> Vec<Option<usize>> {
        static VMASK: Simd<u8, 8_usize> =
            u8x8::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);
        static MASK0: Simd<u32, 8_usize> =
            u32x8::from_array([0x00_00_00_0F; OrderThree::CONSTRAINT_VECTORS]);
        static MASK1: Simd<u32, 8_usize> =
            u32x8::from_array([0x00_00_00_F0; OrderThree::CONSTRAINT_VECTORS]);
        static MASK2: Simd<u32, 8_usize> =
            u32x8::from_array([0x00_00_0F_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK3: Simd<u32, 8_usize> =
            u32x8::from_array([0x00_00_F0_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK4: Simd<u32, 8_usize> =
            u32x8::from_array([0x00_0F_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK5: Simd<u32, 8_usize> =
            u32x8::from_array([0x00_F0_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK6: Simd<u32, 8_usize> =
            u32x8::from_array([0x0F_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK7: Simd<u32, 8_usize> =
            u32x8::from_array([0xF0_00_00_00; OrderThree::CONSTRAINT_VECTORS]);

        let mut a: Simd<u32, 8_usize> = u32x8::from_array([
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(2),
        ]);

        let b: Simd<u32, 8_usize> = u32x8::from_array([
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(4),
        ]);

        let c: Simd<u32, 8_usize> = u32x8::from_array([
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
        ]);

        let add_b = a + b;
        a = ((add_b & MASK0) + c) & MASK0
            | ((add_b & MASK1) + c) & MASK1
            | ((add_b & MASK2) + c) & MASK2
            | ((add_b & MASK3) + c) & MASK3
            | ((add_b & MASK4) + c) & MASK4
            | ((add_b & MASK5) + c) & MASK5
            | ((add_b & MASK6) + c) & MASK6
            | ((add_b & MASK7) + c) & MASK7;

        let a_arr = *a.as_array();

        let mut unpacked = [[0u8; OrderThree::CONSTRAINT_VECTORS]; 8];
        for (index, packed_elem) in a_arr.into_iter().enumerate() {
            let a = unpack_u4x8(packed_elem);
            for i in 0..8 {
                unpacked[i][index] = a[i];
            }
        }

        let mut result = vec![];
        for (index, item) in unpacked.into_iter().enumerate() {
            if u8x8::from_array(item) == VMASK {
                result.push(Some(self.index[index]));
            } else {
                result.push(None);
            }
        }
        result
    }
}

impl CheckCompressed<OrderThree> for CompressedPermutation<u64, 16, OrderThree> {
    unsafe fn unsafe_check_strict(&mut self) -> Vec<Option<usize>> {
        static VMASK: Simd<u8, 8_usize> =
            u8x8::from_array([OrderThree::MAGIC_SUM as u8; OrderThree::CONSTRAINT_VECTORS]);
        static MASK0: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_00_00_00_0F; OrderThree::CONSTRAINT_VECTORS]);
        static MASK1: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_00_00_00_F0; OrderThree::CONSTRAINT_VECTORS]);
        static MASK2: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_00_00_0F_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK3: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_00_00_F0_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK4: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_00_0F_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK5: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_00_F0_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK6: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_0F_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK7: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_00_F0_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK8: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_0F_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK9: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_00_F0_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK10: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_0F_00_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK11: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_00_F0_00_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK12: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_0F_00_00_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK13: Simd<u64, 8_usize> =
            u64x8::from_array([0x00_F0_00_00_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK14: Simd<u64, 8_usize> =
            u64x8::from_array([0x0F_00_00_00_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);
        static MASK15: Simd<u64, 8_usize> =
            u64x8::from_array([0xF0_00_00_00_00_00_00_00; OrderThree::CONSTRAINT_VECTORS]);

        let mut a: Simd<u64, 8_usize> = u64x8::from_array([
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(0),
            *self.square.get_unchecked(2),
        ]);

        let b: Simd<u64, 8_usize> = u64x8::from_array([
            *self.square.get_unchecked(1),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(3),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(4),
            *self.square.get_unchecked(4),
        ]);

        let c = u64x8::from_array([
            *self.square.get_unchecked(2),
            *self.square.get_unchecked(5),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
            *self.square.get_unchecked(7),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(8),
            *self.square.get_unchecked(6),
        ]);

        let add_b = a + b;
        a = ((add_b & MASK0) + c) & MASK0
            | ((add_b & MASK1) + c) & MASK1
            | ((add_b & MASK2) + c) & MASK2
            | ((add_b & MASK3) + c) & MASK3
            | ((add_b & MASK4) + c) & MASK4
            | ((add_b & MASK5) + c) & MASK5
            | ((add_b & MASK6) + c) & MASK6
            | ((add_b & MASK7) + c) & MASK7
            | ((add_b & MASK8) + c) & MASK8
            | ((add_b & MASK9) + c) & MASK9
            | ((add_b & MASK10) + c) & MASK10
            | ((add_b & MASK11) + c) & MASK11
            | ((add_b & MASK12) + c) & MASK12
            | ((add_b & MASK13) + c) & MASK13
            | ((add_b & MASK14) + c) & MASK14
            | ((add_b & MASK15) + c) & MASK15;

        let a_arr = *a.as_array();

        let mut unpacked = [[0u8; OrderThree::CONSTRAINT_VECTORS]; 16];
        for (index, packed_elem) in a_arr.into_iter().enumerate() {
            let a = unpack_u4x16(packed_elem);
            for i in 0..16 {
                unpacked[i][index] = a[i];
            }
        }

        let mut result = vec![];
        for (index, item) in unpacked.into_iter().enumerate() {
            if u8x8::from_array(item) == VMASK {
                result.push(Some(self.index[index]));
            } else {
                result.push(None);
            }
        }
        result
    }
}

#[cfg(test)]
mod test_check {
    use super::*;

    #[test]
    fn check_packed_2x() {
        let p = Permutation::<OrderThree>::permutation_range(69073, 69075);
        let mut b = CompressedPermutation::<u8, 2, OrderThree>::compress_two_from_p_iter(p);
        let c = unsafe { b.unsafe_check_strict() };
        let r = c.into_iter().filter_map(|i| i).collect::<Vec<_>>();
        assert_eq!(vec![69074], r)
    }

    #[test]
    fn check_packed_8x() {
        let p = Permutation::<OrderThree>::permutation_range(69073, 69081);
        let mut b = CompressedPermutation::<u8, 8, OrderThree>::compress_eight_from_p_iter(p);
        let c = unsafe { b.unsafe_check_strict() };
        let r = c.into_iter().filter_map(|i| i).collect::<Vec<_>>();
        assert_eq!(vec![69074], r)
    }

    #[test]
    fn check_packed_16x() {
        let p = Permutation::<OrderThree>::permutation_range(69073, 69089);
        let mut b = CompressedPermutation::<u64, 16, OrderThree>::compress_sixteen_from_p_iter(p);
        let c = unsafe { b.unsafe_check_strict() };
        let r = c.into_iter().filter_map(|i| i).collect::<Vec<_>>();
        assert_eq!(vec![69074], r)
    }
}
