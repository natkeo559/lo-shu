#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{read_serial, write_serial};
use lo_shu::{CheckVector, Enumerable, Permutation, O4};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use itertools::Itertools;
use std::collections::BTreeSet;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    // ========================================================================================= //
    // Read input data from file.                                                                //
    //                                                                                           //
    // Files:                                                                                    //
    // - examples/collected/orderfour/Census.txt (All Magic Squares)                             //
    // - examples/collected/orderfour/UniqueCensus.txt (All Unique Magic Squares)                //
    // - examples/collected/orderfour/Part.txt (Simulated Output of MPSC Solver)                 //
    // - examples/collected/orderfour/Reduced.txt (Pruned Set, Produces Census)                  //
    //                                                                                           //
    // Ordinarily, data from these files comes directly from mpsc threads into the following     //
    // functions. To simplify the flow of data, result from mpsc were cached into Part.txt.      //
    // From this file, the others are generated.                                                 //
    // ========================================================================================= //
    let file: BTreeSet<u64> = read_serial("examples/collected/orderfour/UniqueCensus.txt").unwrap();

    let magic_squares = file
        .into_iter()
        // ========================================================================================= //
        // As more elements are taken, the computation time increases.                           //
        // To generate all magic squares from an input file, take:                               //
        //     Part.txt:            >= 440                                                       //
        //     Census.txt:          >= 447                                                       //
        //     UniqueCensus.txt:    >= 239                                                       //
        //     Reduced.txt:         >= 50                                                        //
        // ===================================================================================== //
        .take(239)
        // ===================================================================================== //
        .map(|a| Permutation::<O4>::kth(a))
        .collect::<BTreeSet<_>>();

    println!("Input Size: {}", magic_squares.len());

    // ========================================================================================= //
    // Filter the set of magic squares from file to contain only unique elements up to rotations //
    // and reflections.                                                                          //
    // ========================================================================================= //
    let mut unique_set = unique_squares(&magic_squares);

    println!("(Unique): {}", unique_set.len());
    println!("Collecting Actions...");

    // ========================================================================================= //
    // Collect Actions                                                                           //
    // ========================================================================================= //
    let actions = compute_group_actions(&unique_set);

    // ========================================================================================= //
    // Filter the set of actions to contain only unique elements up to rotations and             //
    // reflections.                                                                              //
    // ========================================================================================= //
    let unique_actions = unique_squares(&actions);

    println!("Actions: {}", unique_actions.len());
    println!("Extending...");

    // ========================================================================================= //
    // Extend Magic Square Set                                                                   //
    //                                                                                           //
    // For (s, a) in the cartesian product of magic squares and unique actions, compute the      //
    // action of the dihedral group (D8) on s. This should result in the trivial set of          //
    // arrangements of s.                                                                        //
    //                                                                                           //
    // For x in the trivial set, if xa is magic then insert xa into a local set "magic".         //
    // If xa is not magic, then insert a into a local set "reject".                              //
    //                                                                                           //
    // Let each thread manage their own local sets, which are collected later to prevent data    //
    // races and shared memory contention. Generally this is easier to compute since there are   //
    // many threads performing computations on indepenent partitions.                            //
    // ========================================================================================= //
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
                if let Some(m) = (s * a).check_v() {
                    Some(m.index())
                } else {
                    reject.insert(a.index());
                    None
                }
            }));
            (magic, reject)
        })
        .unzip();

    // ========================================================================================= //
    // Collect and unpack the zipped data into its respective set.                               //
    //                                                                                           //
    // Extend the set of unique magic squares with those in the collection                       //
    // If a square in the extension is already in the set of unique magic squares, then          //
    // insertion is skipped. There will be no duplicates in the resulting sets.                  //
    // ========================================================================================= //
    unique_set.extend(
        zipped
            .0
            .into_par_iter()
            .flatten()
            .map(|a| Permutation::<O4>::kth(a))
            .collect::<BTreeSet<_>>(),
    );
    let reject = zipped
        .1
        .into_par_iter()
        .flatten()
        .map(|a| Permutation::<O4>::kth(a))
        .collect();

    // ========================================================================================= //
    // Ensure that the resulting set contains only unique elements up to rotations and           //
    // reflections.                                                                              //
    // ========================================================================================= //
    let unique_set = unique_squares(&unique_set);

    println!("|M| = {}", unique_set.len());
    println!("|A| = {}", unique_actions.len());

    // ========================================================================================= //
    // Set difference between unique actions and the rejection set, leaving only actions that DO //
    // result in a magic square, for all magic squares.                                          //
    // f: M x G -> M                                                                             //
    // ========================================================================================= //
    let g = unique_actions
        .difference(&reject)
        .cloned()
        .collect::<BTreeSet<_>>();

    println!("|G| = {}", g.len());

    let elapsed = start.elapsed();
    println!("Completed in {} seconds", elapsed.as_secs());

    // ========================================================================================= //
    // Write file containing the unique census of order 4 magic squares                          //
    // ========================================================================================= //
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

    // ========================================================================================= //
    // Write file containing unique actions in the set G                                         //
    // ========================================================================================= //
    if g.len() > 3 {
        write_serial(
            g.into_iter().map(|i| i.index()).collect::<BTreeSet<_>>(),
            "examples/collected/orderfour/G.txt",
        )
        .unwrap();
    }
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

/// Computes the transformations between squares in a set.
///
/// Factored action "b" from a * b = c
///
/// b = (c.inv * a).inv
///
/// # Parameters
/// group: &BTreeSet<Permutation<O4>>
fn compute_group_actions(group: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
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
        let g: BTreeSet<u64> = read_serial("examples/collected/orderfour/G.txt").unwrap();

        let s: BTreeSet<u64> = read_serial("examples/collected/orderfour/Census.txt").unwrap();

        for j in g.into_iter().map(|j| Permutation::<O4>::kth(j)) {
            println!("Action: \n{}", j);
            println!("Order: {}", j.cyclic_notation().order());
            let mut passed = 0;
            let mut failed = 0;
            for i in s.iter().map(|i| Permutation::<O4>::kth(*i)) {
                assert!((i * j).check_v().is_some());
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
