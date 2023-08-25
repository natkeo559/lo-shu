#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
use itertools::Itertools;
use lo_shu::{read_serial, CheckVector, Enumerable, Permutation, O4};
use rand::distributions::Uniform;
use rand_distr::Distribution;
use rayon::prelude::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::collections::{BTreeSet, HashSet};

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
        .map(|(&a, &c)| a.inv() * c)
        .collect::<HashSet<_>>()
}

fn group_actions(samples: HashSet<Permutation<O4>>) -> usize {
    let mut unique_set = unique_squares(&samples);

    let actions = compute_group_actions(&unique_set);

    let unique_actions = unique_squares(&actions);

    let stacked = unique_set
        .iter()
        .cartesian_product(unique_actions.iter())
        .par_bridge()
        .map(|(&s, &a)| {
            let mut magic = BTreeSet::new();

            magic.extend(
                s.generate_d()
                    .into_iter()
                    .filter_map(|s| {
                        if let Some(m) = (s * a).check_v() {
                            Some(m.clone().index())
                        } else {
                            None
                        }
                    })
                    .collect::<BTreeSet<u64>>(),
            );
            magic
        })
        .flatten()
        .collect::<BTreeSet<_>>();

    unique_set.extend(
        stacked
            .into_par_iter()
            .map(|a| Permutation::<O4>::kth(a))
            .collect::<HashSet<_>>(),
    );

    let unique_set = unique_squares(&unique_set);
    unique_set.len()
}

fn main() {
    let lookup: Vec<u64> = read_serial("examples/collected/orderfour/UniqueCensus.txt").unwrap();

    let u = Uniform::new(0usize, 880);
    let mut rng = rand::thread_rng();
    let mut max = 0;
    for s in 0..10000 {
        println!("{}", s);
        let sample = u
            .sample_iter(&mut rng)
            .take(48)
            .map(|i| Permutation::<O4>::kth(lookup[i]))
            .collect::<HashSet<_>>();
        let magic = group_actions(sample);

        if magic > max {
            max = magic
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    println!("{}", max);
}
