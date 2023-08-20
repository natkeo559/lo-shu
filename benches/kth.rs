#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use lo_shu::{O3, Permutation, Enumerable};
use std::time::Duration;

fn next_k(n: usize) {
    let mut id = Permutation::<O3>::identity();
    for _ in 0..n {
        id.next_perm();
    }
}

pub fn kth_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("kth");
    group.sample_size(2500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));

    group.bench_function("kth", |b| {
        b.iter(|| Permutation::<O3>::kth(black_box(1000)))
    });

    group.bench_function("next_k", |b| b.iter(|| black_box(next_k(black_box(1000)))));

    group.finish();
}

pub fn kth_growth_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("kth_growth");
    group.sample_size(30);
    group.noise_threshold(0.03);

    for i in 1..=50 {
        group.bench_with_input(BenchmarkId::from_parameter(&i * 6000), &i, |b, &i| {
            b.iter(|| Permutation::<O3>::kth(black_box(i * 6000)));
        });
    }

    group.finish();
}

criterion_group!(benches, kth_bench, kth_growth_bench);
criterion_main!(benches);
