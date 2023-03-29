#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Check, CheckCompressed, CompressedPermutation, OrderThree, Params, Permutation};
use std::collections::HashSet;
// use std::time::Duration;
fn check_single() {
    let mut a = Permutation::<OrderThree>::kth(69074);
    let r = unsafe { a.unsafe_check_strict() };
    assert_eq!(Some(Permutation::<OrderThree>::kth(69074)), r);
}

fn check_compressed_2x() {
    let p = Permutation::<OrderThree>::permutation_range(69073, 69075);
    let mut b = CompressedPermutation::<u8, 2, OrderThree>::compress_two_from_p_iter(p);
    let c = unsafe { b.unsafe_check_strict() };

    assert_eq!(vec![None, Some(69074)], c)
}

fn check_all_compressed_2x() {
    let x =
        (0..OrderThree::PERMUTATIONS / 2)
            .map(|a| unsafe {
                CompressedPermutation::<u8, 2, OrderThree>::compress_two_from_p_iter(
                    Permutation::<OrderThree>::permutation_range(2 * a, 2 * a + 2),
                )
                .unsafe_check_strict()
            })
            .flatten()
            .filter_map(|i| i)
            .collect();
    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

fn check_all_compressed_4x() {
    let x =
        (0..OrderThree::PERMUTATIONS / 4)
            .map(|a| unsafe {
                CompressedPermutation::<u8, 4, OrderThree>::compress_four_from_p_iter(
                    Permutation::<OrderThree>::permutation_range(4 * a, 4 * a + 4),
                )
                .unsafe_check_strict()
            })
            .flatten()
            .filter_map(|i| i)
            .collect();
    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

fn check_all_compressed_8x() {
    let x = (0..OrderThree::PERMUTATIONS / 8)
        .map(|a| unsafe {
            CompressedPermutation::<u8, 8, OrderThree>::compress_eight_from_p_iter(
                Permutation::<OrderThree>::permutation_range(8 * a, 8 * a + 8),
            )
            .unsafe_check_strict()
        })
        .flatten()
        .filter_map(|i| i)
        .collect();
    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

fn check_all_compressed_16x() {
    let x = (0..OrderThree::PERMUTATIONS / 16)
        .map(|a| unsafe {
            CompressedPermutation::<u8, 16, OrderThree>::compress_sixteen_from_p_iter(
                Permutation::<OrderThree>::permutation_range(16 * a, 16 * a + 16),
            )
            .unsafe_check_strict()
        })
        .flatten()
        .filter_map(|i| i)
        .collect();
    assert!(HashSet::from([69074, 77576, 135289, 157120, 205759, 227590, 285303, 293805]) == x);
}

pub fn check_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check");
    group.sample_size(1000);
    group.noise_threshold(0.03);

    group.bench_function("check_single", |b| b.iter(|| black_box(check_single())));

    group.bench_function("check_compressed_2x", |b| {
        b.iter(|| black_box(check_compressed_2x()))
    });

    group.bench_function("check_all_compressed_2x", |b| {
        b.iter(|| black_box(check_all_compressed_2x()))
    });
    group.bench_function("check_all_compressed_4x", |b| {
        b.iter(|| black_box(check_all_compressed_4x()))
    });
    group.bench_function("check_all_compressed_8x", |b| {
        b.iter(|| black_box(check_all_compressed_8x()))
    });
    group.bench_function("check_all_compressed_16x", |b| {
        b.iter(|| black_box(check_all_compressed_16x()))
    });

    group.finish();
}

criterion_group!(benches, check_bench);
criterion_main!(benches);
