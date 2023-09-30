#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{read_serial, O5};
use lo_shu::{Enumerable, Permutation};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use itertools::Itertools;
use std::collections::BTreeSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let file: BTreeSet<u128> = read_serial("examples/collected/orderfive/siamese.txt").unwrap();

    let magic_squares = file
        .into_iter()
        // .take(239)
        .map(|a| Permutation::<O5>::kth(a))
        .collect::<BTreeSet<_>>();

    println!("Input Size: {}", magic_squares.len());

    let mut unique_set = unique_squares(&magic_squares);

    println!("(Unique): {}", unique_set.len());
    println!("Collecting Actions...");

    let actions = compute_group_actions(&unique_set);

    let unique_actions = unique_squares(&actions);

    println!("Actions: {}", unique_actions.len());
    println!("Extending...");

    let zipped: (BTreeSet<_>, BTreeSet<_>) = unique_set
        .iter()
        .cartesian_product(
            unique_actions
                .iter()
                .filter(|&a| *a != Permutation::identity()),
        )
        .par_bridge()
        .map(|(&s, &a)| {
            let mut magic = BTreeSet::new();
            let mut reject = BTreeSet::new();

            magic.extend(s.generate_d().into_iter().filter_map(|s| {
                if let Some(m) = (s * a).check_n_s() {
                    Some(m.index())
                } else {
                    reject.insert(a.index());
                    None
                }
            }));
            (magic, reject)
        })
        .unzip();

    unique_set.extend(
        zipped
            .0
            .into_par_iter()
            .flatten()
            .map(|a| Permutation::<O5>::kth(a))
            .collect::<BTreeSet<_>>(),
    );
    let reject = zipped
        .1
        .into_par_iter()
        .flatten()
        .map(|a| Permutation::<O5>::kth(a))
        .collect();

    let unique_set = unique_squares(&unique_set);

    println!("|M| = {}", unique_set.len());
    println!("|A| = {}", unique_actions.len());

    let g = unique_actions
        .difference(&reject)
        .cloned()
        .collect::<BTreeSet<_>>();

    println!("|G| = {}", g.len());

    let elapsed = start.elapsed();
    println!("Completed in {} seconds", elapsed.as_secs());
}

/// Filters a set to contain unique squares up to all possible rotations and reflections.
///
/// # Parameters:
/// origin: &BTreeSet<Permutation<O4>>
fn unique_squares(origin: &BTreeSet<Permutation<O5>>) -> BTreeSet<Permutation<O5>> {
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

/// Computes the transformations between squares in a set.
///
/// Factored action "b" from a * b = c
///
/// b = (c.inv * a).inv
///
/// # Parameters
/// group: &BTreeSet<Permutation<O4>>
fn compute_group_actions(group: &BTreeSet<Permutation<O5>>) -> BTreeSet<Permutation<O5>> {
    group
        .iter()
        .cartesian_product(group.iter())
        .par_bridge()
        .map(|(&a, &c)| a.inv() * c)
        .collect::<BTreeSet<_>>()
}

#[cfg(test)]
mod debugging {
    use super::*;

    #[test]
    #[ignore = "Debugging"]
    fn dbg_g() {
        let file: Vec<u128> = read_serial("examples/collected/orderfive/siamese.txt").unwrap();

        for i in file.into_iter().map(|i| Permutation::<O5>::kth(i)) {
            println!("{}", i)
        }
    }
}
