use crate::{params::Params, OrderThree, Permutation, PackedPermutation};
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
    unsafe fn unsafe_check_strict(&mut self) -> Option<Permutation<T, P>>;
}

pub trait CheckPacked<T: Clone + Copy, P: Params>
where
    [(); P::ELEMENTS]:,
{
    /// Checks if Permutation of element type T, order N is magic.
    ///
    ///
    /// # Safety
    ///
    /// Use of `get_unchecked` is unsafe. For a safe abstraction, use `get` to return references to Square elements.
    unsafe fn unsafe_check_strict(&mut self);
}


impl Check<u8, OrderThree> for Permutation<u8, OrderThree> {
    unsafe fn unsafe_check_strict(&mut self) -> Option<Permutation<u8, OrderThree>> {
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

impl CheckPacked<u8, OrderThree> for PackedPermutation<u8, OrderThree>{
    unsafe fn unsafe_check_strict(&mut self) {
        
    }
}
