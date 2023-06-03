#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{collections::HashSet, sync::mpsc, thread};

use lo_shu::{Group, OrderThree, Params, Permutation};

fn message_solver() {
    let (sx, rx) = mpsc::channel();

    for i in 0..16 {
        let sender = sx.clone();
        thread::spawn(move || {
            compute(i, sender);
        });
    }
    //80867885530

    match rx.recv() {
        Ok(idxs) => {
            assert_eq!(8, idxs.len());
        }
        Err(_) => panic!("Worker threads disconnected before solution found!"),
    }
}

fn compute(start: usize, sender: mpsc::Sender<HashSet<usize>>) {
    for n in (start..OrderThree::PERMUTATIONS).step_by(16) {
        if let Some(sol) = Permutation::<OrderThree>::kth(n).check_v() {
            match sender.send(sol.generate_d_indexes()) {
                Ok(_) => {}
                Err(_) => println!(
                    "Receiver has stopped listening, dropping worker # {}",
                    start
                ),
            }
            return;
        };
    }
    return;
}

fn main() {
    message_solver();
}
