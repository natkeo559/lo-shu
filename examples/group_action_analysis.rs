#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{CheckVector, Group, OrderFour, Permutation};
use std::collections::HashSet;
use std::fs::{read_to_string, File};
use std::io::Write;

fn main() {
    let mut magic = HashSet::new();
    let mut actions = HashSet::new();
    let mut valid = HashSet::new();
    let mut exclude = HashSet::new();

    let f = read_to_string("examples/collected/OrderFourPart.txt")
        .expect("Could not find input file")
        .lines()
        .take(445)
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for i in f.iter() {
        for j in f.iter() {
            let a = Permutation::<OrderFour>::kth(*i);
            let c = Permutation::<OrderFour>::kth(*j);
            let b = (c.inv() * a).inv();

            // Only take one unique element from b's dihedral group this makes `actions`
            // only contain unique elements up to rotations and reflections, which speeds
            // up computation by reducing duplicated work
            if actions
                .intersection(&b.generate_d_indexes())
                .map(|i| *i)
                .collect::<HashSet<usize>>()
                .is_empty()
            {
                actions.insert(b.index);
            }
        }
    }

    // if an action `b` applied to `a` results in a magic square `s`, then add `b` to `valid`
    // if `s` is magic and not already in the set of known magic squares, add `s` to `magic`
    for i in f.iter() {
        for j in actions.iter() {
            let square = Permutation::<OrderFour>::kth(*i);
            let action = Permutation::<OrderFour>::kth(*j);

            for sqx in square.generate_d() {
                magic.insert(sqx.index);

                if let Some(m) = (sqx * action).check_v() {
                    valid.insert(action);
                    magic.extend(m.generate_d_indexes());
                }
            }
        }
    }

    // once all magic squares have been found
    // For all Magic Squares `s` in the set `M`, group actions `b` in the set `A`,
    // where `M`,`A` are subsets of the Permutation Group `P`,
    // G: {b | s * b is in `M`}
    for i in magic.iter() {
        for j in valid.iter() {
            let a = Permutation::<OrderFour>::kth(*i);
            let b = *j;

            if !magic.contains(&(a * b).index) {
                exclude.insert(b.index);
            }
        }
    }
    let g = actions
        .difference(&exclude)
        .map(|i| *i)
        .collect::<HashSet<usize>>();

    println!("First {} Magic Squares:", f.len());
    println!("|M| = {}", magic.len());
    println!("|A| = {}", valid.len());
    println!("|G| = {}", g.len());

    if magic.len() == 7040 {
        let mut outfile = File::create("examples/collected/OrderFourAll.txt").unwrap();
        let mut magic = magic.into_iter().collect::<Vec<_>>();
        magic.sort();
        for i in magic.iter() {
            write!(outfile, "{}\n", i).unwrap();
        }
    }
}
