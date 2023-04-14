#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::collections::HashSet;

use lo_shu::{Check, Group, OrderThree, Params, Permutation};
use rayon::prelude::*;

fn order_three_linear() -> HashSet<Permutation<OrderThree>> {
    (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .filter_map(|k| Permutation::<OrderThree>::kth(k).check())
        .collect()
}

fn order_three_dihedral() -> HashSet<Permutation<OrderThree>> {
    (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .find_map_first(|i| Permutation::<OrderThree>::kth(i).check())
        .unwrap()
        .generate_d()
}

fn main() {
    let a = order_three_linear();

    let b = order_three_dihedral();

    assert_eq!(a, b);

    for i in b {
        println!("{:?}", i)
    }
}
