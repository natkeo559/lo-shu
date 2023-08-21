#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{fs::read_to_string, collections::BTreeSet};

use itertools::Itertools;
use lo_shu::{Permutation, OrderFour, CheckVector};



fn main() {
    let initial_set: BTreeSet<usize> = read_to_string("examples/collected/orderfour/demo/ExampleInput.txt")
        .expect("Could not find input file")
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<BTreeSet<usize>>();

    let result_set: BTreeSet<usize> = read_to_string("examples/collected/orderfour/demo/ExampleResult.txt")
        .expect("Could not find input file")
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<BTreeSet<usize>>();

    for i in result_set {
        let p = Permutation::<OrderFour>::kth(i);
        assert!(p.check_v().is_some());
    }

    let actions_from_initial_set: BTreeSet<usize> = read_to_string("examples/collected/orderfour/demo/ExampleActions.txt")
        .expect("Could not find input file")
        .lines()
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<BTreeSet<usize>>();

    // for s in initial_set.iter() {
    //     for a in actions_from_initial_set.iter() {
    //         let square = Permutation::<OrderFour>::kth(*s);
    //         let action = Permutation::<OrderFour>::kth(*a);

    //         println!("\n\nSquare: {}\n", square);

    //         println!("Action: {}\n", action);

    //         println!("magic_square: {}", square.check_v().is_some());
    //         println!("magic_action: {}", action.check_v().is_some());
    //         println!("magic_composition: {}", (square * action).check_v().is_some());
    //         println!("in_init: {}\n", initial_set.contains(&(square * action).index));

    //         println!("INV ACTION\n");
    //         println!("magic_composition: {}", (square * action.inv()).check_v().is_some());
    //         println!("in_init: {}\n", initial_set.contains(&(square * action.inv()).index));

    //         println!("INV COMPOSITION\n");
    //         println!("magic_composition: {}", (square * action).inv().check_v().is_some());
    //         println!("in_init: {}", initial_set.contains(&(square * action).inv().index));

    //         println!("invsq: {}", square.inv().check_v().is_some());
    //         println!("---------------------");
    //     }
    // }

    let c = initial_set.iter().cartesian_product(actions_from_initial_set.iter());

    for i in c {
        let square = Permutation::<OrderFour>::kth(*i.0);
        let action = Permutation::<OrderFour>::kth(*i.1);

        for square_d in square.generate_d_indexes().into_iter().map(|p| Permutation::<OrderFour>::kth(p)) {
            if let Some(valid) = (square_d * action).check_v() {
                println!("{:13}: {:15} * {:15} = {:15} in_set: {}", square.index, square_d.index, action.index, valid.index, !initial_set.intersection(&valid.generate_d_indexes()).collect_vec().is_empty());
            }
        }
    }

    println!("\n\nMAGIC SQUARE\n{}", Permutation::<OrderFour>::kth(595512439750));
    println!("\nMAGIC SQUARE\n{}", Permutation::<OrderFour>::kth(2001606418604));
    println!("------------\n");

    let a = Permutation::<OrderFour>::kth(20327277448249);
    println!("(X-AXIS REFLECTION)\n{}\n{}\n", a, a.cyclic_notation());
    
    let b = Permutation::<OrderFour>::kth(12888859194005);
    println!("*\n\n(ACTION FROM 595512439750 TO 2001606418604)\n{}\n{}\n", b, b.cyclic_notation());

    let c = Permutation::<OrderFour>::kth(18921183469395);
    println!("=\n\n(NEW MAGIC SQUARE)\n{}\n{}\n", c, c.cyclic_notation());
}
