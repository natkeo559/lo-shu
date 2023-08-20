#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::thread;

use lo_shu::{CheckVector, O3, Permutation, Enumerable};

#[inline]
fn solve_t_v1() -> BTreeSet<u32> {
    const THREADS: usize = 16;
    const COMPS: usize = 362880 / THREADS;

    let results = Arc::new(Mutex::new(BTreeSet::new()));
    let a = Arc::new(Mutex::new(Permutation::<O3>::identity()));
    let mut handles = vec![];

    for _ in 0..THREADS {
        let a = a.clone();
        let r = results.clone();
        let handle = thread::spawn(move || {
            for _ in 0..COMPS {
                let mut perm = a.lock().unwrap();
                let mut res = r.lock().unwrap();

                match (*perm).next_perm() {
                    Some(p) => match p.check_v() {
                        Some(x) => {
                            res.insert(x.clone().index());
                        }
                        None => (),
                    },
                    None => break,
                };
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let ret = results.lock().unwrap().clone();
    ret
}

#[inline]
fn solve_t_v2() -> BTreeSet<u32> {
    const THREADS: usize = 16;

    let results = Arc::new(Mutex::new(BTreeSet::new()));
    let mut handles = vec![];

    for t in 1..=THREADS {
        let r = results.clone();
        let handle = thread::spawn(move || {
            let local_res = (t..362880)
                .step_by(t)
                .filter_map(|a| Permutation::<O3>::kth(a.try_into().unwrap()).check_v())
                .map(|b| b.clone().index());
            let mut global_res = r.lock().unwrap();
            for i in local_res {
                global_res.insert(i);
            }
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let ret = results.lock().unwrap().clone();
    ret
}

#[inline]
fn main() {
    assert!(solve_t_v1().len() == 8);
    assert!(solve_t_v2().len() == 8);
}
