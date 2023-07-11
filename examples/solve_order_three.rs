#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::CheckVector;
use std::collections::HashSet;

use lo_shu::{OrderThree, Params, Permutation};
use rayon::prelude::*;

fn order_three_linear() -> HashSet<Permutation<OrderThree>> {
    (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .filter_map(|k| Permutation::<OrderThree>::kth(k).check_v())
        .collect()
}

fn order_three_dihedral() -> HashSet<Permutation<OrderThree>> {
    (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .find_map_first(|i| Permutation::<OrderThree>::kth(i).check_v())
        .unwrap()
        .generate_d()
}

fn main() {
    let a = order_three_linear();

    let b = order_three_dihedral();
    println!("A: {} B: {}", a.len(), b.len());

    assert_eq!(a, b);
}
