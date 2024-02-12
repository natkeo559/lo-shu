#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::prelude::*;
use lo_shu::read_serial;
use lo_shu::{CheckVector, Enumerable, Permutation, O4};

use itertools::Itertools;
use std::collections::BTreeSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file: BTreeSet<u64> = read_serial("examples/collected/orderfour/Reduced.txt").unwrap();

    let magic_squares = file
        .into_iter()
        .take(50)
        .map(|a| Permutation::<O4>::kth(a))
        .collect::<BTreeSet<_>>();

    println!("Input Size: {}", magic_squares.len());

    let mut unique_set = reduce_isometry(&magic_squares);

    println!("(Unique): {}", unique_set.len());
    println!("Collecting Actions...");

    let actions = compute_factors_ainvc(&unique_set);

    println!("Extending...");

    let mut ext = BTreeSet::new();
    unique_set
        .iter()
        .cartesian_product(actions.iter())
        .for_each(|(&a, &b)| {
            for iso_e in b.generate_d() {
                if let Some(m) = (a * iso_e.inv()).check_v() {
                    ext.insert(m);
                }
            }
        });

    unique_set.extend(&ext);

    let unique_set = unique_squares(&unique_set);

    println!("|M| = {}", unique_set.len());

    let elapsed = start.elapsed();
    println!("Completed in {} seconds", elapsed.as_secs());
}

/// Filters a set to contain unique squares up to all possible rotations and reflections.
///
/// # Parameters:
/// origin: &BTreeSet<Permutation<O4>>
fn unique_squares(origin: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    let mut unique_set = BTreeSet::new();
    for s in origin.iter() {
        if unique_set
            .intersection(&s.generate_d().into_iter().collect())
            .map(|i| *i)
            .collect::<BTreeSet<_>>()
            .is_empty()
        {
            unique_set.insert(*s);
        }
    }
    unique_set
}
