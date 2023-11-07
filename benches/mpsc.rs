#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::{
    sync::mpsc::{self, Sender},
    thread,
};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Enumerable, IndexConst, Permutation, O3};

pub fn from_builder(t: usize) -> Result<(), anyhow::Error> {
    let (sx, rx) = mpsc::channel();

    for i in 0..t {
        let sender: Sender<Permutation<O3>> = sx.clone();
        thread::spawn(move || {
            for n in (i as u32..O3::MAX_INDEX).step_by(t) {
                if let Some(sol) = Permutation::<O3>::kth(n).check_n_s() {
                    match sender.send(sol) {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
            }
            return;
        });
    }

    drop(sx);
    let mut res = vec![];
    let mut recv_iter = rx.iter();
    for _ in 0..8 {
        match recv_iter.next() {
            Some(idxs) => res.push(idxs),
            None => break,
        }
    }

    assert_eq!(res.len(), 8);
    Ok(())
}

pub fn mpsc_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("mpsc");

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
            b.iter(|| black_box(from_builder(black_box(i.0))))
        });
    }

    group.finish();
}

criterion_group!(benches, mpsc_bench);
criterion_main!(benches);
