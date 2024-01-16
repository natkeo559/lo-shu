#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{
    CheckScalar, CheckVector, Construction, Enumerable, ParameterSetError, Permutation, O3, O4, O5,
};
use std::time::Duration;

fn check_unsafe_vector() {
    let a = Permutation::<O3>::kth(69074);
    let r = unsafe { a.check_v_unsafe() };
    assert_eq!(Some(Permutation::<O3>::kth(69074)), r);
}

fn check_safe_vector() {
    let a = Permutation::<O3>::kth(69074);
    let r = a.check_v();
    assert_eq!(Some(Permutation::<O3>::kth(69074)), r);
}

fn check_unsafe_scalar() {
    let a = Permutation::<O3>::kth(69074);
    let r = unsafe { a.check_s_unsafe() };
    assert_eq!(Some(Permutation::<O3>::kth(69074)), r);
}

fn check_safe_scalar() {
    let a = Permutation::<O3>::kth(69074);
    let r = a.check_s();
    assert_eq!(Some(Permutation::<O3>::kth(69074)), r);
}

fn check_generic_three_v() {
    let a = Construction::<O3>::siamese(1);
    let r = a.check_n_v::<8>();
    assert_eq!(Some(a), r);
}

fn check_generic_four_v() -> Result<(), ParameterSetError> {
    let c = Construction::try_from(Permutation::<O4>::kth(80867885530))?;
    let r = c.check_n_v::<16>();
    assert_eq!(Some(c), r);

    Ok(())
}

fn check_generic_five_v() {
    let a = Construction::<O5>::siamese(2);
    let r = a.check_n_v::<16>();
    assert_eq!(Some(a), r);
}

fn check_generic_three_s() {
    let a = Construction::<O3>::siamese(1);
    let r = a.check_n_s();
    assert_eq!(Some(a), r);
}

fn check_generic_four_s() -> Result<(), ParameterSetError> {
    let c = Construction::try_from(Permutation::<O4>::kth(80867885530))?;
    let r = c.check_n_s();
    assert_eq!(Some(c), r);

    Ok(())
}

fn check_generic_five_s() {
    let a = Construction::<O5>::siamese(2);
    let r = a.check_n_s();
    assert_eq!(Some(a), r);
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

pub fn check_generic_bench_v(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_generic_v");
    group.sample_size(10500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("order_three", |b| b.iter(black_box(check_generic_three_v)));
    group.bench_function("order_four", |b| b.iter(black_box(check_generic_four_v)));
    group.bench_function("order_five", |b| b.iter(black_box(check_generic_five_v)));
    group.finish();
}

pub fn check_generic_bench_s(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_generic_s");
    group.sample_size(10500);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("order_three", |b| b.iter(black_box(check_generic_three_s)));
    group.bench_function("order_four", |b| b.iter(black_box(check_generic_four_s)));
    group.bench_function("order_five", |b| b.iter(black_box(check_generic_five_s)));
    group.finish();
}

criterion_group!(
    benches,
    check_v_bench,
    check_s_bench,
    // check_safe_s_v_bench,
    // check_unsafe_s_v_bench,
    // check_generic_bench_v,
    // check_generic_bench_s
);
criterion_main!(benches);
