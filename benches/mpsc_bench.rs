#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{thread, sync::{Arc, atomic::AtomicBool, mpsc::{self, Sender}}};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Group, OrderThree, Permutation};
use lo_shu::{ThreadManager, Worker};

fn message_solver() {
    let f = Arc::new(AtomicBool::new(false));
    let (sx, rx) = mpsc::channel();

    for i in 0..16 {
        let sender: Sender<Permutation<OrderThree>> = sx.clone();
        let found = f.clone();
        let tm = ThreadManager {threads: 16, poll: true, polling_rate: 16};
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

pub fn mpsc_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpsc");
    group.sample_size(3000);
    group.noise_threshold(0.03);

    group.bench_function("order_three", |b| b.iter(|| black_box(message_solver())));

    group.finish();
}

criterion_group!(benches, mpsc_bench);
criterion_main!(benches);
