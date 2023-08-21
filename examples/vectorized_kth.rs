#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(portable_simd)]
#![feature(iter_array_chunks)]

// pub fn kth(k: usize) -> Self {
//     let ogk = k;
//     let mut n = Self::identity();
//     let mut indeces = [0; P::ELEMENTS];

use std::simd::{Simd, SimdOrd, SimdPartialEq};

use lo_shu::{Params, Permutation, O3};

pub fn kth(k: usize) -> Permutation<O3> {
    let mut n = Permutation::identity();
    let mut indeces = [0; O3::ELEMENTS];

    let mut divisor = 1;
    for place in 1..=O3::ELEMENTS {
        if k / divisor != 0 {
            indeces[O3::ELEMENTS - place] = (k / divisor) % place;
            divisor *= place;
        }
    }

    for (i, item) in indeces.iter().enumerate() {
        let index = item + i;
        if index != i {
            let temp = n.square[index];
            let mut j = index;
            while j > i {
                n.square[j] = n.square[j - 1];
                j -= 1;
            }
            n.square[i] = temp;
        }
    }
    n
}

fn v_kth(k_v: Vec<usize>) {
    // let nv = vec![Permutation::<O3>::identity(); 8];
    let vs = Simd::from_slice(&k_v[..]);
    let mut divisors: Simd<usize, 8> = Simd::splat(1);
    let mut place_div = divisors;

    for place in 1..=O3::ELEMENTS {
        let m_mask = (vs / divisors).simd_ne(Simd::splat(0));
        let m = m_mask.select(divisors, Simd::splat(0));
        place_div = m.simd_max(place_div);
        divisors *= Simd::splat(place);
    }
}

fn main() {
    v_kth(vec![400, 555, 12234, 1212, 1, 101010, 90, 30000]);
}
