use rayon::prelude::*;
use std::simd::*;

pub fn iterate_perms() {
    let x = (0..362890)
        .into_par_iter()
        .filter_map(|k| check(kth_perm(k)))
        .collect::<Vec<_>>();

    println!("{:?}", x);
}

pub fn kth_perm(k: i32) -> [u8; 9] {
    /*
    Based on:
    https://stackoverflow.com/questions/31216097/given-n-and-k-return-the-kth-permutation-sequence
    */

    let mut n = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut indeces = [0; 9];

    let mut divisor = 1;
    for place in 1..10 {
        if k / divisor == 0 {
            break;
        }
        indeces[9 - place] = (k / divisor) % place as i32;
        divisor *= place as i32;
    }
    for i in 0..9 {
        let index = indeces[i] as usize + i;
        if index != i {
            let temp = n[index];
            let mut j = index;
            while j > i {
                n[j] = n[j - 1];
                j -= 1;
            }
            n[i] = temp;
        }
    }
    n
}

#[inline(never)]
pub fn check(v: [u8; 9]) -> Option<[u8; 9]> {
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
