use itertools::Itertools;
use std::simd::*;

fn main() {
    iterate_perms();
}

pub fn iterate_perms() -> Vec<Vec<u8>> {
    (1u8..10)
        .permutations(9)
        .chunks(2)
        .into_iter()
        .filter_map(|chunk| check(chunk.flatten().collect::<Vec<u8>>()))
        .collect::<Vec<_>>()
}

// #[inline(never)]
pub fn check(v: Vec<u8>) -> Option<Vec<u8>> {
    const VSUM: Simd<u8, 16_usize> = u8x16::from_array([
        15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    ]);

    let mut a: Simd<u8, 16_usize> = u8x16::from_array(unsafe {
        [
            *v.get_unchecked(0),
            *v.get_unchecked(3),
            *v.get_unchecked(6),
            *v.get_unchecked(0),
            *v.get_unchecked(1),
            *v.get_unchecked(2),
            *v.get_unchecked(0),
            *v.get_unchecked(2),
            *v.get_unchecked(9),
            *v.get_unchecked(12),
            *v.get_unchecked(15),
            *v.get_unchecked(9),
            *v.get_unchecked(10),
            *v.get_unchecked(11),
            *v.get_unchecked(9),
            *v.get_unchecked(11),
        ]
    });

    let mut b: Simd<u8, 16_usize> = u8x16::from_array(unsafe {
        [
            *v.get_unchecked(1),
            *v.get_unchecked(4),
            *v.get_unchecked(7),
            *v.get_unchecked(3),
            *v.get_unchecked(4),
            *v.get_unchecked(5),
            *v.get_unchecked(4),
            *v.get_unchecked(4),
            *v.get_unchecked(10),
            *v.get_unchecked(13),
            *v.get_unchecked(16),
            *v.get_unchecked(12),
            *v.get_unchecked(13),
            *v.get_unchecked(14),
            *v.get_unchecked(15),
            *v.get_unchecked(13),
        ]
    });

    a += &b;

    b = u8x16::from_array(unsafe {
        [
            *v.get_unchecked(2),
            *v.get_unchecked(5),
            *v.get_unchecked(8),
            *v.get_unchecked(6),
            *v.get_unchecked(7),
            *v.get_unchecked(8),
            *v.get_unchecked(8),
            *v.get_unchecked(6),
            *v.get_unchecked(11),
            *v.get_unchecked(14),
            *v.get_unchecked(17),
            *v.get_unchecked(15),
            *v.get_unchecked(16),
            *v.get_unchecked(17),
            *v.get_unchecked(17),
            *v.get_unchecked(15),
        ]
    });

    a += &b;

    match a.simd_eq(VSUM).to_array() {
        [true, true, true, true, true, true, true, true, _, _, _, _, _, _, _, _] => {
            Some(v[0..9].to_owned())
        }
        [_, _, _, _, _, _, _, _, true, true, true, true, true, true, true, true] => {
            Some(v[9..16].to_owned())
        }
        _ => None,
    }
}