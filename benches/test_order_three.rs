#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{ThreadManager, Worker};
use std::collections::HashSet;
use std::{
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

use lo_shu::{Check, Group, OrderFour, OrderThree, Params, Permutation};
use rayon::prelude::*;

fn order_three_linear() -> HashSet<Permutation<OrderThree>> {
    (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .filter_map(|k| Permutation::<OrderThree>::kth(k).check())
        .collect()
}

fn order_three_dihedral() -> HashSet<Permutation<OrderThree>> {
    (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .find_map_first(|i| Permutation::<OrderThree>::kth(i).check())
        .unwrap()
        .generate_d()
}

// fn message_solver() {
//     let (sx, rx) = mpsc::channel();

//     for i in 0..16 {
//         let sender = sx.clone();
//         thread::spawn(move || {
//             compute(i + 80867885530, sender);
//         });
//     }
//     //80867885530
//     let mut sols: Vec<usize> = Vec::new();
//     for _ in 0..16 {
//         match rx.recv() {
//             Ok(idxs) => {
//                 println!("{}", Permutation::<OrderFour>::kth(idxs));
//                 sols.push(idxs);
//             }
//             Err(_) => panic!("Worker threads disconnected before solution found!"),
//         }
//     }

//     println!("{:?}", sols);
// }

fn message_solver(t: usize) {
    let f = Arc::new(AtomicBool::new(false));
    let (sx, rx) = mpsc::channel();

    for i in 0..t {
        let sender: Sender<Permutation<OrderFour>> = sx.clone();
        let found = f.clone();
        let tm = ThreadManager::new(t, 1, true);
        thread::spawn(move || {
            tm.channel_check(i, sender, found);
        });
    }

    match rx.recv() {
        Ok(idxs) => {
            assert_eq!(8, idxs.generate_d_indexes().len());
        }
        Err(_) => panic!("Worker threads disconnected before solution found!"),
    }
}

// fn compute(start: usize, sender: mpsc::Sender<usize>) {
//     for n in (start..OrderFour::PERMUTATIONS).step_by(16) {
//         if let Some(sol) = Permutation::<OrderFour>::kth(n).check() {
//             match sender.send(sol.index) {
//                 Ok(_) => {}
//                 Err(_) => println!(
//                     "Receiver has stopped listening, dropping worker # {}",
//                     start
//                 ),
//             }
//         };
//     }
//     return;
// }

fn main() {
    let a = order_three_linear();

    let b = order_three_dihedral();
    println!("A: {} B: {}", a.len(), b.len());

    assert_eq!(a, b);

    message_solver(16);
}
