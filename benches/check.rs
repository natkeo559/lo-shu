#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{CheckScalar, CheckVector, OrderThree, Permutation};
use std::time::Duration;

fn check_unsafe_vector() {
    let a = Permutation::<OrderThree>::kth(69074);
    let r = unsafe { a.check_v_unsafe() };
    assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
}

fn check_safe_vector() {
    let a = Permutation::<OrderThree>::kth(69074);
    let r = a.check_v();
    assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
}

fn check_unsafe_scalar() {
    let a = Permutation::<OrderThree>::kth(69074);
    let r = unsafe { a.check_s_unsafe() };
    assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
}

fn check_safe_scalar() {
    let a = Permutation::<OrderThree>::kth(69074);
    let r = a.check_s();
    assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
}

pub fn check_v_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_v");
    group.sample_size(10500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("unsafe", |b| b.iter(black_box(check_unsafe_vector)));
    group.bench_function("safe", |b| b.iter(black_box(check_safe_vector)));
    group.finish();
}

pub fn check_s_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_s");
    group.sample_size(10500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("unsafe", |b| b.iter(black_box(check_unsafe_scalar)));
    group.bench_function("safe", |b| b.iter(black_box(check_safe_scalar)));
    group.finish();
}

pub fn check_safe_s_v_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_safe_s_v");
    group.sample_size(10500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("scalar", |b| b.iter(black_box(check_safe_scalar)));
    group.bench_function("vector", |b| b.iter(black_box(check_safe_vector)));
    group.finish();
}

pub fn check_unsafe_s_v_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_unsafe_s_v");
    group.sample_size(10500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("scalar", |b| b.iter(black_box(check_unsafe_scalar)));
    group.bench_function("vector", |b| b.iter(black_box(check_unsafe_vector)));
    group.finish();
}

criterion_group!(
    benches,
    check_v_bench,
    check_s_bench,
    check_safe_s_v_bench,
    check_unsafe_s_v_bench
);
criterion_main!(benches);
