use std::collections::BTreeSet;

use lo_shu::{read_serial, Permutation, Enumerable, O4, write_serial};

// To make output work with GAP, delete all quotes
fn main() {
    let file: BTreeSet<u64> = read_serial("examples/collected/orderfour/Reduced.txt").unwrap();

    let output = file
        .into_iter()
        .map(|i| format!("{}", Permutation::<O4>::kth(i)
        .cyclic_notation())).collect::<Vec<String>>();

    write_serial(output, "examples/collected/orderfour/ReducedCycles.txt").unwrap();
}