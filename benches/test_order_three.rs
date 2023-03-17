#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{OrderThree, Permutation, Check};
use rayon::prelude::*;

fn main() {
    let b = (0..362879)
        .into_par_iter()
        .filter_map(|k| unsafe { Permutation::<u8, OrderThree>::kth(k).unsafe_check_strict() })
        .map(|r| r.index)
        .collect::<Vec<_>>();

    println!("{:?}",b)
}