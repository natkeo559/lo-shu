#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use std::collections::BTreeSet;
use std::fs::read_to_string;
use itertools::Itertools;
use lo_shu::{CheckVector, OrderFour, Permutation};
use rand::distributions::Uniform;
use rand_distr::Distribution;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};

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

            (a.inv() * c).index
        })
        .collect::<BTreeSet<usize>>()
}

fn group_actions(samples: BTreeSet<usize>) -> usize {
    let mut unique_set = unique_squares(&samples);

    let actions = compute_group_actions(&unique_set);

    let unique_actions: BTreeSet<usize> = unique_squares(&actions);

    let stacked: BTreeSet<_> = unique_set
        .iter()
        .cartesian_product(unique_actions.iter())
        .par_bridge()
        .map(|(&s, &a)| {
            let mut magic = BTreeSet::new();
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
                            None
                        }
                    })
                    .collect::<BTreeSet<usize>>(),
            );
            magic
        }).collect();

    unique_set.extend(stacked.into_par_iter().flatten().collect::<BTreeSet<_>>());
    
    let unique_set = unique_squares(&unique_set);
    unique_set.len()
}

fn main() {
    let lookup = read_to_string("examples/collected/orderfour/UniqueCensus.txt")
            .expect("Could not find input file")
            .lines()
            .map(|line| line.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

    let u = Uniform::new(0usize, 880);
    let mut rng = rand::thread_rng();
    let mut max = 0;
    for s in 0..10000 {
        println!("{}", s);
        let sample = u.sample_iter(&mut rng).take(4)
            .map(|i|lookup[i]).collect::<BTreeSet<usize>>();
        let magic = group_actions(sample);

        if magic > max {
            max = magic
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    println!("{}", max);
}