#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::collections::{BTreeSet, HashMap, HashSet};

use itertools::Itertools;
use lo_shu::{Enumerable, Permutation, O4, read_serial, read_file, write_serial};

fn main() {
    let magic_squares: BTreeSet<u64> = read_serial("examples/collected/orderfour/Reduced.txt").unwrap();

    let mut cycle_map = HashMap::new();

    for i in magic_squares {
        let mut cycles = Permutation::<O4>::kth(i).cyclic_notation();

        let cycle_lens = cycles.order();
        if cycle_map.contains_key(&cycle_lens) {
            let value: &mut Vec<Permutation<O4>> = cycle_map.get_mut(&cycle_lens).unwrap();
            value.push(cycles.into_permutation())
        } else {
            cycle_map.insert(cycle_lens.clone(), vec![cycles.into_permutation()]);
        }
    }
    let mut sorted = HashSet::new();
    for i in cycle_map.iter() {
        sorted.insert(i.1.clone());
    }

    let k = cycle_map.keys().sorted().rev();
    let mut sum = 0;
    for i in k {
        sum += cycle_map.get(i).unwrap().len();
        println!("{} : {}", i, cycle_map.get(i).unwrap().len());
    }
    println!("{}", sum)
}
