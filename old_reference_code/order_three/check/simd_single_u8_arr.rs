use std::simd::*;

pub fn simd_single_u8_arr(v: [u8; 9]) -> Option<[u8; 9]> {
    const VMASK: Simd<u8, 8_usize> = u8x8::from_array([15, 15, 15, 15, 15, 15, 15, 15]);

    let mut a: Simd<u8, 8_usize> = u8x8::from_array(unsafe {
        [
            *v.get_unchecked(0),
            *v.get_unchecked(3),
            *v.get_unchecked(6),
            *v.get_unchecked(0),
            *v.get_unchecked(1),
            *v.get_unchecked(2),
            *v.get_unchecked(0),
            *v.get_unchecked(2),
        ]
    });

    let mut b: Simd<u8, 8_usize> = u8x8::from_array(unsafe {
        [
            *v.get_unchecked(1),
            *v.get_unchecked(4),
            *v.get_unchecked(7),
            *v.get_unchecked(3),
            *v.get_unchecked(4),
            *v.get_unchecked(5),
            *v.get_unchecked(4),
            *v.get_unchecked(4),
        ]
    });

    a += &b;

    b = u8x8::from_array(unsafe {
        [
            *v.get_unchecked(2),
            *v.get_unchecked(5),
            *v.get_unchecked(8),
            *v.get_unchecked(6),
            *v.get_unchecked(7),
            *v.get_unchecked(8),
            *v.get_unchecked(8),
            *v.get_unchecked(6),
        ]
    });

    a += &b;

    match a == VMASK {
        true => Some(v),
        false => None,
    }
}
