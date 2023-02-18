use crate::Permutation;
use std::simd::*;

pub trait Check<T: Clone + Copy> {
    fn check_simd_single(&mut self) -> Option<Permutation<T>>;
}

impl Check<u16> for Permutation<u16> {
    fn check_simd_single(&mut self) -> Option<Permutation<u16>> {
        const VMASK: Simd<u16, 8_usize> = u16x8::from_array([15, 15, 15, 15, 15, 15, 15, 15]);

        let mut a: Simd<u16, 8_usize> = u16x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(2),
            ]
        });

        let mut b: Simd<u16, 8_usize> = u16x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(4),
            ]
        });

        a += &b;

        b = u16x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
            ]
        });

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl Check<u8> for Permutation<u8> {
    fn check_simd_single(&mut self) -> Option<Permutation<u8>> {
        const VMASK: Simd<u8, 8_usize> = u8x8::from_array([15, 15, 15, 15, 15, 15, 15, 15]);

        let mut a: Simd<u8, 8_usize> = u8x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(2),
            ]
        });

        let mut b: Simd<u8, 8_usize> = u8x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(4),
            ]
        });

        a += &b;

        b = u8x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
            ]
        });

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl Check<f32> for Permutation<f32> {
    fn check_simd_single(&mut self) -> Option<Permutation<f32>> {
        const VMASK: Simd<f32, 8_usize> =
            f32x8::from_array([15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0]);

        let mut a: Simd<f32, 8_usize> = f32x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(2),
            ]
        });

        let mut b: Simd<f32, 8_usize> = f32x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(4),
            ]
        });

        a += &b;

        b = f32x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
            ]
        });

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl Check<f64> for Permutation<f64> {
    fn check_simd_single(&mut self) -> Option<Permutation<f64>> {
        const VMASK: Simd<f64, 8_usize> =
            f64x8::from_array([15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0]);

        let mut a: Simd<f64, 8_usize> = f64x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(0),
                *self.square.array.get_unchecked(2),
            ]
        });

        let mut b: Simd<f64, 8_usize> = f64x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(1),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(3),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(4),
                *self.square.array.get_unchecked(4),
            ]
        });

        a += &b;

        b = f64x8::from_array(unsafe {
            [
                *self.square.array.get_unchecked(2),
                *self.square.array.get_unchecked(5),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
                *self.square.array.get_unchecked(7),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(8),
                *self.square.array.get_unchecked(6),
            ]
        });

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}
