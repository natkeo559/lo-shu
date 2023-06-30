#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(portable_simd)]
#![feature(iter_array_chunks)]

use lo_shu::{OrderThree, Params, Permutation};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use std::simd::{u8x64, u8x8, Simd};

fn main() {
    let perm_arr = (0..OrderThree::PERMUTATIONS)
        .map(|k| Permutation::<OrderThree>::kth(k).square.0)
        .array_chunks::<8>()
        .par_bridge();
    const VMASK: Simd<u8, 8> = u8x8::from_slice(&[15; 8]);

    let result: Vec<_> = perm_arr
        .filter_map(|i| {
            let mut elems_0 = [0; 64];
            let mut elems_1 = [0; 64];
            let mut elems_2 = [0; 64];

            for n in 0..i.len() {
                elems_0[0 + (8 * n)] = i[n][0];
                elems_0[1 + (8 * n)] = i[n][1];
                elems_0[2 + (8 * n)] = i[n][2];
                elems_0[3 + (8 * n)] = i[n][0];
                elems_0[4 + (8 * n)] = i[n][3];
                elems_0[5 + (8 * n)] = i[n][6];
                elems_0[6 + (8 * n)] = i[n][0];
                elems_0[7 + (8 * n)] = i[n][2];

                elems_1[0 + (8 * n)] = i[n][3];
                elems_1[1 + (8 * n)] = i[n][4];
                elems_1[2 + (8 * n)] = i[n][5];
                elems_1[3 + (8 * n)] = i[n][1];
                elems_1[4 + (8 * n)] = i[n][4];
                elems_1[5 + (8 * n)] = i[n][7];
                elems_1[6 + (8 * n)] = i[n][4];
                elems_1[7 + (8 * n)] = i[n][4];

                elems_2[0 + (8 * n)] = i[n][6];
                elems_2[1 + (8 * n)] = i[n][7];
                elems_2[2 + (8 * n)] = i[n][8];
                elems_2[3 + (8 * n)] = i[n][2];
                elems_2[4 + (8 * n)] = i[n][5];
                elems_2[5 + (8 * n)] = i[n][8];
                elems_2[6 + (8 * n)] = i[n][8];
                elems_2[7 + (8 * n)] = i[n][6];
            }
            let sums_0 = u8x64::from_slice(&elems_0);
            let sums_1 = u8x64::from_slice(&elems_1);
            let sums_2 = u8x64::from_slice(&elems_2);

            let res = sums_0 + sums_1 + sums_2;
            let mut valid = vec![];
            for (idx, r) in res
                .to_array()
                .chunks_exact(8)
                .map(|a| u8x8::from_slice(a))
                .enumerate()
            {
                if r == VMASK {
                    valid.push(i[idx])
                }
            }
            if !valid.is_empty() {
                Some(valid)
            } else {
                None
            }
        })
        .collect();

    for i in result {
        println!("{:?}", i)
    }
}
