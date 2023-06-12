#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{OrderFour, Permutation};
use std::collections::HashSet;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("Reading File");
    let f: Vec<usize> = read_to_string("examples/collected/OrderFourAll.txt")
        .expect("Could not find input file")
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    println!("Collecting Actions");
    let actions: Vec<usize> = read_to_string("examples/collected/OrderFourRightUnfiltered.txt")
        .expect("Could not find input file")
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect();

    let mut exclude: HashSet<usize> = HashSet::new();

    println!("Filtering Actions");
    for i in f.iter() {
        for j in actions.iter() {
            let square = Permutation::<OrderFour>::kth(*i);
            let action = Permutation::<OrderFour>::kth(*j);

            if !f.contains(&(square * action).index) {
                exclude.insert(*j);
            }
        }
    }

    println!("Reducing G");
    let g = actions
        .into_iter()
        .collect::<HashSet<_>>()
        .difference(&exclude)
        .map(|i| *i)
        .collect::<HashSet<usize>>();

    println!("First {} Magic Squares:", f.len());
    println!("|M| = {}", f.len());
    println!("|G| = {}", g.len());

    let mut outfile = File::create("examples/collected/OrderFourActions.txt").unwrap();
    let mut act = g.into_iter().collect::<Vec<_>>();
    act.sort();
    for i in act.iter() {
        write!(outfile, "{}\n", i).unwrap();
    }
}

#[cfg(test)]
mod group_actions {
    use super::*;

    #[test]
    #[ignore = "Debugging"]
    fn morphism() {
        println!("Reading File");
        let a: Vec<usize> = read_to_string("examples/collected/OrderFourActions.txt")
            .expect("Could not find input file")
            .lines()
            .map(|line| line.trim().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        for i in Permutation::<OrderFour>::indexes(a.as_slice()) {
            println!("{}", i)
        }
    }
}
