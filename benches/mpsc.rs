#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Enumerable, Permutation, O3};
use lo_shu::{ThreadManager, Worker};
use std::{
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Sender},
        Arc,
    },
    thread,
};

fn message_solver(t: usize) {
    let f = Arc::new(AtomicBool::new(false));
    let (sx, rx) = mpsc::channel();

    for i in 0..t {
        let sender: Sender<Permutation<O3>> = sx.clone();
        let found = f.clone();
        let tm = ThreadManager::new(t, 1, true);
        thread::spawn(move || {
            tm.channel_check(i, sender, found);
        });
    }

    match rx.recv() {
        Ok(idxs) => {
            assert_eq!(
                8,
                idxs.generate_d()
                    .into_iter()
                    .map(|a| a.clone().index())
                    .len()
            );
        }
        Err(_) => panic!("Worker threads disconnected before solution found!"),
    }
}

pub fn mpsc_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpsc");
    group.sample_size(3000);
    group.noise_threshold(0.03);

    let input = [
        (1, "order_three_1"),
        (2, "order_three_2"),
        (4, "order_three_4"),
        (8, "order_three_8"),
        (16, "order_three_16"),
        (32, "order_three_32"),
        (64, "order_three_64"),
    ];
    for inp in input {
        group.bench_with_input(inp.1, &inp, |b, i| {
            b.iter(|| message_solver(black_box(i.0)))
        });
    }

    group.finish();
}

criterion_group!(benches, mpsc_bench);
criterion_main!(benches);
