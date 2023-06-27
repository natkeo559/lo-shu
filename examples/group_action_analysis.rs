#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{CheckVector, OrderFour, Permutation};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use itertools::Itertools;
use std::collections::{BTreeSet, HashSet};
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

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
    let mut magic_squares = read_to_string("examples/collected/orderfour/Census.txt")
        .expect("Could not find input file")
        .lines()
        .take(1100)
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<BTreeSet<usize>>();

    // for i in [87357715922,8099169412855,8228242280293] {
    //     magic_squares.insert(i);
    // }

    println!("Input Size: {}", magic_squares.len());

    // Filter the set of magic squares from file to contain only unique elements up to rotations and reflections.
    let mut unique_set = BTreeSet::new();
    for s in magic_squares.iter() {
        if unique_set
            .intersection(&Permutation::<OrderFour>::kth(*s).generate_d_indexes())
            .map(|i| *i)
            .collect::<BTreeSet<usize>>()
            .is_empty()
        {
            unique_set.insert(*s);
        }
    }

    println!("(Unique): {}", unique_set.len());
    println!("Collecting Actions...");

    // Collect Actions
    // Actions: {b | a * b = c for all a,c in M of magic squares}
    let actions = magic_squares
        .iter()
        .cartesian_product(magic_squares.iter())
        .par_bridge()
        .map(|(&i, &j)| {
            let a = Permutation::<OrderFour>::kth(i);
            let c = Permutation::<OrderFour>::kth(j);

            // Factored action "b" from a * b = c
            // b = (c.inv * a).inv
            // Collect these into a set.
            (c.inv() * a).inv().index
        })
        .collect::<BTreeSet<usize>>();

    // Filter the set of actions to contain only unique elements up to rotations and reflections.
    let mut unique_actions = BTreeSet::new();
    actions.iter().for_each(|&a| {
        let action = Permutation::<OrderFour>::kth(a);
        if unique_actions
            .intersection(&action.generate_d_indexes())
            .cloned()
            .collect::<HashSet<usize>>()
            .is_empty()
        {
            unique_actions.insert(a);
        }
    });

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
    let zipped: (BTreeSet<_>, BTreeSet<_>) = magic_squares
        .iter()
        .cartesian_product(unique_actions.iter())
        .par_bridge()
        .map(|(&s, &a)| {
            let mut magic = BTreeSet::new();
            let mut reject = BTreeSet::new();
            let square = Permutation::<OrderFour>::kth(s);
            let action = Permutation::<OrderFour>::kth(a);

            magic.extend(square.generate_d_indexes());
            magic.extend(
                square
                    .generate_d()
                    .into_iter()
                    .filter_map(|s| {
                        if let Some(m) = (s * action).check_v() {
                            Some(m.generate_d_indexes())
                        } else {
                            reject.insert(action.index);
                            None
                        }
                    })
                    .flatten()
                    .collect::<BTreeSet<usize>>(),
            );
            (magic, reject)
        })
        .unzip();

    // Unpack the zipped data into its respective set.
    magic_squares.extend(zipped.0.into_par_iter().flatten().collect::<BTreeSet<_>>());
    let reject = zipped.1.into_par_iter().flatten().collect::<BTreeSet<_>>();

    println!("|M| = {}", magic_squares.len());
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
    if magic_squares.len() == 7040 {
        let mut outfile = File::create("examples/collected/orderfour/Census.txt").unwrap();
        for i in magic_squares.iter() {
            write!(outfile, "{}\n", i).unwrap();
        }
    }

    // if !g.is_empty() {
    //     let mut outfile = File::create("examples/collected/orderfour/G.txt").unwrap();
    //     for i in g.iter() {
    //         write!(outfile, "{}\n", i).unwrap();
    //     }
    // }
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

        for i in g.into_iter().map(|i| Permutation::<OrderFour>::kth(i)) {
            println!("{}", i.cyclic_notation().order())
        }
    }
}
