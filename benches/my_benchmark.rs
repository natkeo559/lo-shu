#![feature(imported_main)]

use criterion::{criterion_group, criterion_main, Criterion};
use lo_shu::order_three::permute::*;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simd_threaded_kth_u8", |b| {
        b.iter(|| simd_threaded_kth_u8::iterate_perms())
    });
    c.bench_function("simd_perms_u8", |b| {
        b.iter(|| simd_perms_u8::iterate_perms())
    });
    c.bench_function("simd_double_perms_u8", |b| {
        b.iter(|| simd_double_perms_u8::iterate_perms())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
