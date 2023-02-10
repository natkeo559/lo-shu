use std::simd::*;

use rayon::prelude::*;

use crate::order_three::permute::simd_threaded_kth_u8::kth_perm;

pub fn try_something() -> Vec<[u8; 9]> {
    (0..362890)
        .into_par_iter()
        .filter_map(|k| check(kth_perm(k)))
        .collect::<Vec<_>>()
}

pub fn check(mut v: [u8; 9]) -> Option<[u8; 9]> {
    v.par_iter_mut().for_each(|i| *i %= 3);

    const VMASK: Simd<u8, 8_usize> = u8x8::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);

    let mut a: Simd<u8, 8_usize> = u8x8::from_slice(unsafe {
        &[
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

    let mut b: Simd<u8, 8_usize> = u8x8::from_slice(unsafe {
        &[
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

    b = u8x8::from_slice(unsafe {
        &[
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

    a.as_mut_array().par_iter_mut().for_each(|i| *i %= 3);

    match a == VMASK {
        true => Some(v),
        false => None,
    }
}

#[cfg(test)]
mod mod_tests {
    use super::*;

    #[test]
    fn test_try() {
        let r = try_something();
        println!("{:?}", r);
        println!("{:?}", r.len());
    }
}
