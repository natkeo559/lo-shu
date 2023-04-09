#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Check, OrderThree, Params, Permutation};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, time::Duration};

fn solve_order_three_linear() {
    let b: HashSet<usize> = (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .filter_map(|k| Permutation::<OrderThree>::kth(k).check())
        .map(|i| i.index)
        .collect();

    assert!(b.len() == 8)
}

pub fn order_three_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_three");
    group.sample_size(2500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(15, 0));

    group.bench_function("solve_linear", |b| b.iter(solve_order_three_linear));

    group.bench_function("kth", |b| {
        b.iter(|| Permutation::<OrderThree>::kth(black_box(1000)))
    });

    group.finish();
}

criterion_group!(benches, order_three_bench);
criterion_main!(benches);
