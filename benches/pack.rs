#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{
    pack_u4x16, pack_u4x2, pack_u4x4, pack_u4x8, CompressedPermutation, OrderThree, Permutation,
};

fn pack_two_bench() {
    let _a = pack_u4x2([15, 15]);
}

fn pack_four_bench() {
    let _a = pack_u4x4([15, 15, 15, 15]);
}

fn pack_eight_bench() {
    let _a = pack_u4x8([15, 15, 15, 15, 15, 15, 15, 15]);
}

fn pack_sixteen_bench() {
    let _a = pack_u4x16([
        15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15,
    ]);
}

fn pack_two_from_iter_bench() {
    let x = Permutation::<OrderThree>::permutation_range(0, 2);
    let _a = CompressedPermutation::<u8, 2, OrderThree>::compress_two_from_p_iter(x);
}

fn pack_four_from_iter_bench() {
    let x = Permutation::<OrderThree>::permutation_range(0, 4);
    let _a = CompressedPermutation::<u8, 4, OrderThree>::compress_four_from_p_iter(x);
}

fn pack_eight_from_iter_bench() {
    let x = Permutation::<OrderThree>::permutation_range(0, 8);
    let _a = CompressedPermutation::<u8, 8, OrderThree>::compress_eight_from_p_iter(x);
}

fn pack_sixteen_from_iter_bench() {
    let x = Permutation::<OrderThree>::permutation_range(0, 16);
    let _a = CompressedPermutation::<u8, 16, OrderThree>::compress_sixteen_from_p_iter(x);
}

pub fn pack_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("compress");
    group.sample_size(1000);
    group.noise_threshold(0.03);

    group.bench_function("two", |b| {
        b.iter(|| {
            pack_two_bench();
            black_box(())
        })
    });

    group.bench_function("four", |b| {
        b.iter(|| {
            pack_four_bench();
            black_box(())
        })
    });

    group.bench_function("eight", |b| {
        b.iter(|| {
            pack_eight_bench();
            black_box(())
        })
    });

    group.bench_function("sixteen", |b| {
        b.iter(|| {
            pack_sixteen_bench();
            black_box(())
        })
    });

    group.bench_function("two_iter", |b| {
        b.iter(|| {
            pack_two_from_iter_bench();
            black_box(())
        })
    });

    group.bench_function("four_iter", |b| {
        b.iter(|| {
            pack_four_from_iter_bench();
            black_box(())
        })
    });

    group.bench_function("eight_iter", |b| {
        b.iter(|| {
            pack_eight_from_iter_bench();
            black_box(())
        })
    });

    group.bench_function("sixteen_iter", |b| {
        b.iter(|| {
            pack_sixteen_from_iter_bench();
            black_box(())
        })
    });

    group.finish();
}

criterion_group!(benches, pack_bench);
criterion_main!(benches);
