use std::collections::BTreeSet;

use lo_shu::{read_serial, write_serial, Enumerable, Permutation, O4};

// To make output work with GAP, delete all quotation marks from generated file
fn main() {
    let file: BTreeSet<u64> = read_serial("examples/collected/orderfour/Reduced.txt").unwrap();

    let output = file
        .into_iter()
        .map(|i| format!("{}", Permutation::<O4>::kth(i).cyclic_notation()))
        .collect::<Vec<String>>();

    write_serial(&output, "examples/collected/orderfour/ReducedCycles.txt").unwrap();
}
