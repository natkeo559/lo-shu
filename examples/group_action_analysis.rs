#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{CheckVector, OrderFour, Permutation};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use itertools::Itertools;
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

fn unique_squares(origin: &BTreeSet<usize>) -> BTreeSet<usize> {
    let mut unique_set = BTreeSet::new();
    for s in origin.iter() {
        if unique_set
            .intersection(&Permutation::<OrderFour>::kth(*s).generate_d_indexes())
            .map(|i| *i)
            .collect::<BTreeSet<usize>>()
            .is_empty()
        {
            unique_set.insert(*s);
        }
    }
    unique_set
}

fn compute_group_actions(group: &BTreeSet<usize>) -> BTreeSet<usize> {
    group
        .iter()
        .cartesian_product(group.iter())
        .par_bridge()
        .map(|(&i, &j)| {
            let a = Permutation::<OrderFour>::kth(i);
            let c = Permutation::<OrderFour>::kth(j);

            // Factored action "b" from a * b = c
            // b = (c.inv * a).inv
            // Collect these into a set.
            (a.inv() * c).index
        })
        .collect::<BTreeSet<usize>>()
}

fn main() {
    let start = Instant::now();

    // Read input data from file.
    //
    // Files:
    // - examples/collected/orderfour/Census.txt (All Magic Squares)
    // - examples/collected/orderfour/Part.txt (Simulated Output of MPSC Solver)
    //
    // Ordinarily, data from these files comes directly from mpsc threads into the analysis functions.
    // To simplify the flow of data, result from mpsc were cached into Part.txt
    //
    // Part.txt will contain unordered and possibly duplicate (rare) elements due to data races associated with
    // message passing and shared memory contention.
    //
    // Census.txt contains all ordered magic squares as a result from the computations performed on Part.txt
    //
    // As more elements are taken, the computation time increases exponentially. To generate all magic squares
    // from Part.txt, take at least 445 elements. For Census.txt, take at least 447 elements.
    let magic_squares = read_to_string("examples/collected/orderfour/Reduced.txt")
        .expect("Could not find input file")
        .lines()
        .take(50)
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<BTreeSet<usize>>();

    // for i in [87357715922,8099169412855,8228242280293] {
    //     magic_squares.insert(i);
    // }

    println!("Input Size: {}", magic_squares.len());

    // Filter the set of magic squares from file to contain only unique elements up to rotations and reflections.
    let mut unique_set = unique_squares(&magic_squares);

    println!("(Unique): {}", unique_set.len());
    println!("Collecting Actions...");

    // Collect Actions
    // Actions: {b | a * b = c for all a,c in M of magic squares}
    let actions = compute_group_actions(&unique_set);

    // Filter the set of actions to contain only unique elements up to rotations and reflections.
    let unique_actions: BTreeSet<usize> = unique_squares(&actions);

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
            let square = Permutation::<OrderFour>::kth(s);
            let action = Permutation::<OrderFour>::kth(a);

            magic.extend(
                square
                    .generate_d()
                    .into_iter()
                    .filter_map(|s| {
                        if let Some(m) = (s * action).check_v() {
                            Some(m.index)
                        } else {
                            reject.insert(action.index);
                            None
                        }
                    })
                    .collect::<BTreeSet<usize>>(),
            );
            (magic, reject)
        })
        .unzip();

    // Unpack the zipped data into its respective set.
    unique_set.extend(zipped.0.into_par_iter().flatten().collect::<BTreeSet<_>>());
    let reject = zipped.1.into_par_iter().flatten().collect::<BTreeSet<_>>();

    let unique_set = unique_squares(&unique_set);
    println!("|M| = {}", unique_set.len());
    println!("|A| = {}", unique_actions.len());

    // Set difference between unique actions and the rejection set, leaving only actions that DO result in a
    // magic square, for all magic squares.
    // f: M x G -> M
    let g = unique_actions
        .difference(&reject)
        .cloned()
        .collect::<BTreeSet<usize>>();

    println!("|G| = {}", g.len());

    let elapsed = start.elapsed();
    println!("Completed in {} seconds", elapsed.as_secs());

    // Write files containing a census of order 4 magic squares (not unique) and actions in G (unique)
    if unique_set.len() == 880 {
        let mut outfile = File::create("examples/collected/orderfour/UniqueCensus.txt").unwrap();
        for i in unique_set.iter() {
            write!(outfile, "{}\n", i).unwrap();
        }
    }

    if !g.is_empty() {
        let mut outfile = File::create("examples/collected/orderfour/G.txt").unwrap();
        for i in g.iter() {
            write!(outfile, "{}\n", i).unwrap();
        }
    }
}

#[cfg(test)]
mod debugging {
    use super::*;

    #[test]
    #[ignore = "Debugging"]
    fn dbg_g() {
        let g = read_to_string("examples/collected/orderfour/G.txt")
            .expect("Could not find input file")
            .lines()
            .map(|line| line.trim().parse::<usize>().unwrap())
            .collect::<BTreeSet<usize>>();

        let s = read_to_string("examples/collected/orderfour/Census.txt")
            .expect("Could not find input file")
            .lines()
            .map(|line| line.trim().parse::<usize>().unwrap())
            .collect::<BTreeSet<usize>>();

        for j in g.into_iter().map(|j| Permutation::<OrderFour>::kth(j)) {
            println!("Action: \n{}", j);
            println!("Order: {}", j.cyclic_notation().order());
            let mut passed = 0;
            let mut failed = 0;
            for i in s.iter().map(|i| Permutation::<OrderFour>::kth(*i)) {
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
