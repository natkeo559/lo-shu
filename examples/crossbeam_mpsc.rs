use std::thread;

use crossbeam_channel::{self, unbounded};
use lo_shu::{Enumerable, Permutation, O3};

fn main() {
    let (s, r) = unbounded();

    for i in 0..16 {
        let sender = s.clone();
        thread::spawn(move || {
            for n in (i..69074).step_by(16) {
                if let Some(sol) = Permutation::<O3>::kth(n.try_into().unwrap()).check_n_s() {
                    match sender.send(sol) {
                        Ok(_) => {}
                        Err(_) => {}
                    };
                }
            }
        });
    }

    let mut count = 0;
    loop {
        match r.recv() {
            Ok(p) => {
                count += 1;
                println!("{count}");
                println!("{}", p);
            }
            Err(_) => {
                println!("ERR recv")
            }
        }

        if count == 8 {
            break;
        }
    }
}
