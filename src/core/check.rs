use crate::Square;
use std::simd::*;

pub trait Check<T: Clone + Copy> {
    fn check_simd_single(&mut self) -> Option<Square<T>>;
}

impl Check<u8> for Square<u8> {
    fn check_simd_single(&mut self) -> Option<Square<u8>> {
        const VMASK: Simd<u8, 8_usize> = u8x8::from_array([15, 15, 15, 15, 15, 15, 15, 15]);

        let mut a: Simd<u8, 8_usize> = u8x8::from_array(unsafe {
            [
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(3),
                *self.array.get_unchecked(6),
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(1),
                *self.array.get_unchecked(2),
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(2),
            ]
        });

        let mut b: Simd<u8, 8_usize> = u8x8::from_array(unsafe {
            [
                *self.array.get_unchecked(1),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(7),
                *self.array.get_unchecked(3),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(5),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(4),
            ]
        });

        a += &b;

        b = u8x8::from_array(unsafe {
            [
                *self.array.get_unchecked(2),
                *self.array.get_unchecked(5),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(6),
                *self.array.get_unchecked(7),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(6),
            ]
        });

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl Check<f32> for Square<f32> {
    fn check_simd_single(&mut self) -> Option<Square<f32>> {
        const VMASK: Simd<f32, 8_usize> =
            f32x8::from_array([15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0]);

        let mut a: Simd<f32, 8_usize> = f32x8::from_array(unsafe {
            [
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(3),
                *self.array.get_unchecked(6),
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(1),
                *self.array.get_unchecked(2),
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(2),
            ]
        });

        let mut b: Simd<f32, 8_usize> = f32x8::from_array(unsafe {
            [
                *self.array.get_unchecked(1),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(7),
                *self.array.get_unchecked(3),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(5),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(4),
            ]
        });

        a += &b;

        b = f32x8::from_array(unsafe {
            [
                *self.array.get_unchecked(2),
                *self.array.get_unchecked(5),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(6),
                *self.array.get_unchecked(7),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(6),
            ]
        });

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}

impl Check<f64> for Square<f64> {
    fn check_simd_single(&mut self) -> Option<Square<f64>> {
        const VMASK: Simd<f64, 8_usize> =
            f64x8::from_array([15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0, 15.0]);

        let mut a: Simd<f64, 8_usize> = f64x8::from_array(unsafe {
            [
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(3),
                *self.array.get_unchecked(6),
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(1),
                *self.array.get_unchecked(2),
                *self.array.get_unchecked(0),
                *self.array.get_unchecked(2),
            ]
        });

        let mut b: Simd<f64, 8_usize> = f64x8::from_array(unsafe {
            [
                *self.array.get_unchecked(1),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(7),
                *self.array.get_unchecked(3),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(5),
                *self.array.get_unchecked(4),
                *self.array.get_unchecked(4),
            ]
        });

        a += &b;

        b = f64x8::from_array(unsafe {
            [
                *self.array.get_unchecked(2),
                *self.array.get_unchecked(5),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(6),
                *self.array.get_unchecked(7),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(8),
                *self.array.get_unchecked(6),
            ]
        });

        a += &b;

        match a == VMASK {
            true => Some(*self),
            false => None,
        }
    }
}
