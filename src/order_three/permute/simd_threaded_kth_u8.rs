use rayon::prelude::*;
use crate::check::simd_single_u8_arr::simd_single_u8_arr;


pub fn iterate_perms() -> Vec<[u8; 9]> {
    (0..362890)
        .into_par_iter()
        .filter_map(|k| simd_single_u8_arr(kth_perm(k)))
        .collect::<Vec<_>>()
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
