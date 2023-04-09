#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Check, OrderThree, Permutation};
use std::time::Duration;

fn check_unsafe() {
    let a = Permutation::<OrderThree>::kth(69074);
    let r = unsafe { a.check_unsafe() };
    assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
}

fn check() {
    let a = Permutation::<OrderThree>::kth(69074);
    let r = a.check();
    assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
}

pub fn check_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check");
    group.sample_size(10000);
    group.noise_threshold(0.03);

    group.measurement_time(Duration::new(10, 0));

    group.bench_function("unsafe", |b| b.iter(|| black_box(check_unsafe())));

    group.bench_function("safe", |b| b.iter(|| black_box(check())));

    group.finish();
}

criterion_group!(benches, check_bench);
criterion_main!(benches);
