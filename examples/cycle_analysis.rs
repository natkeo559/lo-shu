#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{
    collections::{BTreeSet, HashMap},
    fs::read_to_string,
};

use lo_shu::{OrderFour, Permutation};

fn main() {
    let magic_squares = read_to_string("examples/collected/orderfour/Census.txt")
        .expect("Could not find input file")
        .lines()
        .take(7040)
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<BTreeSet<usize>>();

    let mut cycle_map: HashMap<Vec<usize>, u32> = HashMap::new();

    for i in magic_squares {
        let cycles = Permutation::<OrderFour>::kth(i).cyclic_notation();

        let cycle_lens: Vec<usize> = cycles.k.into_iter().map(|c| c.len()).collect();
        if let Some(n) = cycle_map.insert(cycle_lens.clone(), 1) {
            let value = cycle_map.get_mut(&cycle_lens).unwrap();
            *value = n + 1;
        }
    }
    let mut sorted = BTreeSet::new();
    for i in cycle_map.iter() {
        sorted.insert(i.1.clone());
    }

    println!("{}", sorted.len())
}
