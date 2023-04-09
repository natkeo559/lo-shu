#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{CheckCompressed, CompressedPermutation, OrderThree, Params, Permutation};
use std::{collections::HashSet, time::Duration};

fn check_compressed_2x() {
    let p = Permutation::<OrderThree>::permutation_range(69073, 69075);
    let b = CompressedPermutation::<u8, 2, OrderThree>::compress_two_from_p_iter(p);
    let c = b.check();
    assert_eq!(Some(69074), c)
}

fn check_compressed_4x() {
    let p = Permutation::<OrderThree>::permutation_range(69073, 69077);
    let b = CompressedPermutation::<u8, 4, OrderThree>::compress_four_from_p_iter(p);
    let c = b.check();
    assert_eq!(Some(69074), c)
}

fn check_compressed_8x() {
    let p = Permutation::<OrderThree>::permutation_range(69073, 69081);
    let b = CompressedPermutation::<u16, 8, OrderThree>::compress_eight_from_p_iter(p);
    let c = b.check();
    assert_eq!(Some(69074), c)
}

fn check_compressed_16x() {
    let p = Permutation::<OrderThree>::permutation_range(69073, 69089);
    let b = CompressedPermutation::<u64, 16, OrderThree>::compress_sixteen_from_p_iter(p);
    let c = b.check();
    assert_eq!(Some(69074), c)
}

fn check_all_compressed_2x() {
    let x =
        (0..(OrderThree::PERMUTATIONS / 2) - 2)
            .filter_map(|a| {
                CompressedPermutation::<u8, 2, OrderThree>::compress_two_from_p_iter(
                    Permutation::<OrderThree>::permutation_range(2 * a, 2 * a + 2),
                )
                .check()
            })
            .collect();

    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

fn check_all_compressed_4x() {
    let x =
        (0..(OrderThree::PERMUTATIONS / 4) - 4)
            .filter_map(|a| {
                CompressedPermutation::<u8, 4, OrderThree>::compress_four_from_p_iter(
                    Permutation::<OrderThree>::permutation_range(4 * a, 4 * a + 4),
                )
                .check()
            })
            .collect();

    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

fn check_all_compressed_8x() {
    let x = (0..(OrderThree::PERMUTATIONS / 8) - 8)
        .filter_map(|a| {
            CompressedPermutation::<u8, 8, OrderThree>::compress_eight_from_p_iter(
                Permutation::<OrderThree>::permutation_range(8 * a, 8 * a + 8),
            )
            .check()
        })
        .collect();

    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

fn check_all_compressed_16x() {
    let x = (0..(OrderThree::PERMUTATIONS / 16) - 16)
        .filter_map(|a| {
            CompressedPermutation::<u8, 16, OrderThree>::compress_sixteen_from_p_iter(
                Permutation::<OrderThree>::permutation_range(16 * a, 16 * a + 16),
            )
            .check()
        })
        .collect();

    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

pub fn check_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check_compressed");
    group.sample_size(1000);
    group.noise_threshold(0.03);

    group.measurement_time(Duration::new(10, 0));

    group.bench_function("check_compressed_2x", |b| {
        b.iter(|| black_box(check_compressed_2x()))
    });

    group.bench_function("check_compressed_4x", |b| {
        b.iter(|| black_box(check_compressed_4x()))
    });

    group.bench_function("check_compressed_8x", |b| {
        b.iter(|| black_box(check_compressed_8x()))
    });

    group.bench_function("check_compressed_16x", |b| {
        b.iter(|| black_box(check_compressed_16x()))
    });

    group.measurement_time(Duration::new(20, 0));
    group.bench_function("check_all_compressed_2x", |b| {
        b.iter(|| black_box(check_all_compressed_2x()))
    });

    group.measurement_time(Duration::new(16, 0));
    group.bench_function("check_all_compressed_4x", |b| {
        b.iter(|| black_box(check_all_compressed_4x()))
    });

    group.measurement_time(Duration::new(14, 0));
    group.bench_function("check_all_compressed_8x", |b| {
        b.iter(|| black_box(check_all_compressed_8x()))
    });

    group.measurement_time(Duration::new(13, 0));
    group.bench_function("check_all_compressed_16x", |b| {
        b.iter(|| black_box(check_all_compressed_16x()))
    });

    group.finish();
}

criterion_group!(benches, check_bench);
criterion_main!(benches);
