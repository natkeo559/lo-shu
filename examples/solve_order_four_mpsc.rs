#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use lo_shu::{OrderFour, Permutation};
use lo_shu::{ThreadManager, Worker};
use std::{
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

pub fn message_solver(t: usize) {
    let f = Arc::new(AtomicBool::new(false));
    let (sx, rx) = mpsc::channel();

    for i in 0..t {
        let sender: Sender<Permutation<OrderFour>> = sx.clone();
        let found = f.clone();
        let tm = ThreadManager::new(t, 512, false);
        thread::spawn(move || {
            tm.channel_check(i, sender, found);
        });
    }
    loop {
        match rx.recv() {
            Ok(idxs) => {
                println!("{}", idxs.index);
            }
            Err(_) => panic!("Worker threads disconnected before solution found!"),
        }
    }
}

fn main() {
    message_solver(16);
}
