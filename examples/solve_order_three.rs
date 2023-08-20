#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::CheckVector;
use std::collections::HashSet;

use lo_shu::{O3, Permutation, Enumerable};
use rayon::prelude::*;

fn order_three_linear() -> HashSet<Permutation<O3>> {
    (0..362880)
        .into_par_iter()
        .filter_map(|k| Permutation::<O3>::kth(k).check_v())
        .collect()
}

fn order_three_dihedral() -> HashSet<Permutation<O3>> {
    (0..362880)
        .into_par_iter()
        .find_map_first(|i| Permutation::<O3>::kth(i).check_v())
        .unwrap()
        .generate_d()
}

fn main() {
    let a = order_three_linear();

    let b = order_three_dihedral();
    println!("A: {} B: {}", a.len(), b.len());

    assert_eq!(a, b);
}
