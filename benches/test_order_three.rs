#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{Check, OrderThree, Params, Permutation};
use rayon::prelude::*;

fn main() {
    let b: Vec<Permutation<OrderThree>> = (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .filter_map(|k| Permutation::<OrderThree>::kth(k).check())
        .collect();

    for i in b {
        println!("{:?}", i)
    }
}
