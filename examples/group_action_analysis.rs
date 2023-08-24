#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{read_serial, write_serial};
use lo_shu::{CheckVector, Enumerable, Permutation, O4};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use itertools::Itertools;
use std::collections::BTreeSet;
use std::collections::HashSet;
use std::time::Instant;

fn unique_squares(origin: &HashSet<Permutation<O4>>) -> HashSet<Permutation<O4>> {
    let mut unique_set = HashSet::new();
    for s in origin.iter() {
        if unique_set
            .intersection(&s.generate_d())
            .map(|i| *i)
            .collect::<HashSet<_>>()
            .is_empty()
        {
            unique_set.insert(*s);
        }
    }
    unique_set
}

fn compute_group_actions(group: &HashSet<Permutation<O4>>) -> HashSet<Permutation<O4>> {
    group
        .iter()
        .cartesian_product(group.iter())
        .par_bridge()
        .map(|(&a, &c)| {
            // Factored action "b" from a * b = c
            // b = (c.inv * a).inv
            // Collect these into a set.
            a.inv() * c
        })
        .collect::<HashSet<_>>()
}

fn main() {
    let start = Instant::now();

    // Read input data from file.
    //
    // Files:
    // - examples/collected/O4/Census.txt (All Magic Squares)
    // - examples/collected/O4/Part.txt (Simulated Output of MPSC Solver)
    //
    // Ordinarily, data from these files comes directly from mpsc threads into the analysis functions.
    // To simplify the flow of data, result from mpsc were cached into Part.txt
    //
    // Part.txt will contain unordered elements due to data races associated with
    // multi-threaded message passing and shared memory contention.
    //
    // Census.txt contains all ordered magic squares as a result from the computations performed on Part.txt
    //
    // As more elements are taken, the computation time increases. To generate all magic squares
    // from Part.txt, take at least 445 elements. For Census.txt, take at least 447 elements.
    let file: HashSet<u64> = read_serial("examples/collected/orderfour/Reduced.txt").unwrap();

    let magic_squares = file
        .into_iter()
        .map(|a| Permutation::<O4>::kth(a))
        .collect::<HashSet<_>>();

    println!("Input Size: {}", magic_squares.len());

    // Filter the set of magic squares from file to contain only unique elements up to rotations and reflections.
    let mut unique_set = unique_squares(&magic_squares);

    println!("(Unique): {}", unique_set.len());
    println!("Collecting Actions...");

    // Collect Actions
    // Actions: {b | a * b = c for all a,c in M of magic squares}
    let actions = compute_group_actions(&unique_set);

    // Filter the set of actions to contain only unique elements up to rotations and reflections.
    let unique_actions = unique_squares(&actions);

    println!("Actions: {}", unique_actions.len());
    println!("Extending...");

    // For every element in the cartesian product of magic squares and unique actions,
    // insert the rotations and reflections of a magic square into the set of all magic squares.
    // For s in the set of rotations and reflections of a magic square, if s operated on by an action
    // is a magic square, insert the square along with it's rotations/reflections into the set of
    // all magic squares. This process may produce magic squares that aren't obtained simply by rotating
    // and reflecting.
    //
    // Let each thread manage their own local sets, which are collected later to prevent data races
    // and shared memory contention.
    //
    // Rejection set contains unique actions that do NOT result in a magic square when applied
    // to any other magic square. For example, if a magic square `a` and action `b` resulted in a magic square
    // then apply `b` to all other magic squares. If `b` fails to produce a magic square, reject it - insert it
    // into the rejection set. Think of this as "failure by counter-example".
    let zipped: (BTreeSet<_>, BTreeSet<_>) = unique_set
        .iter()
        .cartesian_product(unique_actions.iter())
        .par_bridge()
        .map(|(&s, &a)| {
            let mut magic = BTreeSet::new();
            let mut reject = BTreeSet::new();

            magic.extend(
                s.generate_d()
                    .into_iter()
                    .filter_map(|s| {
                        if let Some(m) = (s * a).check_v() {
                            Some(m.index())
                        } else {
                            reject.insert(a.index());
                            None
                        }
                    })
                    .collect::<BTreeSet<u64>>(),
            );
            (magic, reject)
        })
        .unzip();

    // Unpack the zipped data into its respective set.
    unique_set.extend(
        zipped
            .0
            .into_par_iter()
            .flatten()
            .map(|a| Permutation::<O4>::kth(a))
            .collect::<HashSet<_>>(),
    );
    let reject = zipped
        .1
        .into_par_iter()
        .flatten()
        .map(|a| Permutation::<O4>::kth(a))
        .collect();

    let unique_set = unique_squares(&unique_set);
    println!("|M| = {}", unique_set.len());
    println!("|A| = {}", unique_actions.len());

    // Set difference between unique actions and the rejection set, leaving only actions that DO result in a
    // magic square, for all magic squares.
    // f: M x G -> M
    let g = unique_actions
        .difference(&reject)
        .cloned()
        .collect::<HashSet<_>>();

    println!("|G| = {}", g.len());

    let elapsed = start.elapsed();
    println!("Completed in {} seconds", elapsed.as_secs());

    // Write files containing a census of order 4 magic squares (not unique) and actions in G (unique)
    if unique_set.len() == 880 {
        write_serial(
            unique_set
                .into_iter()
                .map(|i| i.index())
                .collect::<BTreeSet<_>>(),
            "examples/collected/orderfour/UniqueCensus.txt",
        )
        .unwrap();
    }

    if g.len() > 3 {
        write_serial(
            g.into_iter().map(|i| i.index()).collect::<BTreeSet<_>>(),
            "examples/collected/orderfour/G.txt",
        )
        .unwrap();
    }
}

#[cfg(test)]
mod debugging {
    use super::*;

    #[test]
    #[ignore = "Debugging"]
    fn dbg_g() {
        let g: BTreeSet<u64> = read_serial("examples/collected/orderfour/G.txt").unwrap();

        let s: BTreeSet<u64> = read_serial("examples/collected/orderfour/Census.txt").unwrap();

        for j in g.into_iter().map(|j| Permutation::<O4>::kth(j)) {
            println!("Action: \n{}", j);
            println!("Order: {}", j.cyclic_notation().order());
            let mut passed = 0;
            let mut failed = 0;
            for i in s.iter().map(|i| Permutation::<O4>::kth(*i)) {
                // assert!((j * i).check_v().is_some());
                if (i * j).check_v().is_some() {
                    passed += 1;
                } else {
                    failed += 1;
                }
            }
            println!("Passed: {}\nFailed: {}\n", passed, failed);
        }
    }
}
