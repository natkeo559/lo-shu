#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use std::collections::BTreeSet;
use std::fs::read_to_string;
use itertools::Itertools;
use lo_shu::{CheckVector, OrderFour, Permutation};
use rand::{seq::SliceRandom, thread_rng};
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

fn group_actions(samples: BTreeSet<usize>) -> (usize, usize) {
    let mut magic_squares = samples;

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

    let actions = magic_squares
        .iter()
        .cartesian_product(magic_squares.iter())
        .par_bridge()
        .map(|(&i, &j)| {
            let a = Permutation::<OrderFour>::kth(i);
            let c = Permutation::<OrderFour>::kth(j);
            (c.inv() * a).inv().index
        })
        .collect::<BTreeSet<usize>>();

    let mut unique_actions = BTreeSet::new();
    actions.iter().for_each(|&a| {
        let action = Permutation::<OrderFour>::kth(a);
        if unique_actions
            .intersection(&action.generate_d_indexes())
            .cloned()
            .collect::<BTreeSet<usize>>()
            .is_empty()
        {
            unique_actions.insert(a);
        }
    });

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

    magic_squares.extend(zipped.0.into_par_iter().flatten().collect::<BTreeSet<_>>());

    (magic_squares.len(), unique_actions.len())
}

fn main() {
    let a = read_to_string("examples/collected/orderfour/Census.txt")
        .expect("Could not find input file")
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut max = 0;
    for _ in 0..400 {
        let sample = a.choose_multiple(&mut thread_rng(), 50);
        let samples = sample.into_iter().cloned().collect::<BTreeSet<_>>();
        let (magic, _) = group_actions(samples);

        if magic > max {
            max = magic
        }
    }

    println!("{}", max);
    

}