#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Check, CheckPacked, OrderThree, PackedPermutation, Params, Permutation};
// use std::time::Duration;
fn check_single() {
    let mut a = Permutation::<u8, OrderThree>::kth(69074);
    let r = unsafe { a.unsafe_check_strict() };
    assert_eq!(Some(Permutation::<u8, OrderThree>::kth(69074)), r);
}

fn check_packed_2x() {
    let p = Permutation::<u8, OrderThree>::permutation_range(69073, 69075);
    let mut b = PackedPermutation::<u8, 2, OrderThree>::pack_two_from_p_iter(p);
    let c = unsafe { b.unsafe_check_strict() };

    assert_eq!(vec![None, Some(69074)], c)
}

fn check_all_packed_2x() {
    let mut r = vec![];
    for a in 0..OrderThree::PERMUTATIONS / 2 {
        let p = Permutation::<u8, OrderThree>::permutation_range(2 * a, 2 * a + 2);
        let mut b = PackedPermutation::<u8, 2, OrderThree>::pack_two_from_p_iter(p);
        let c = unsafe { b.unsafe_check_strict() };
        for i in c {
            if i.is_some() {
                r.push(i)
            }
        }
    }
    assert_eq!(
        vec![
            Some(69074),
            Some(77576),
            Some(135289),
            Some(157120),
            Some(205759),
            Some(227590),
            Some(285303),
            Some(293805)
        ],
        r
    );
}

fn check_all_packed_16x() {
    let mut r = vec![];
    for a in 0..OrderThree::PERMUTATIONS / 16 {
        let p = Permutation::<u8, OrderThree>::permutation_range(16 * a, 16 * a + 16);
        let mut b = PackedPermutation::<u8, 16, OrderThree>::pack_sixteen_from_p_iter(p);
        let c = unsafe { b.unsafe_check_strict() };
        for i in c {
            if i.is_some() {
                r.push(i)
            }
        }
    }
    assert_eq!(
        vec![
            Some(69074),
            Some(77576),
            Some(135289),
            Some(157120),
            Some(205759),
            Some(227590),
            Some(285303),
            Some(293805)
        ],
        r
    );
}

pub fn check_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("check");
    group.sample_size(1000);
    group.noise_threshold(0.03);

    group.bench_function("check_single", |b| {
        b.iter(|| {
            check_single();
            black_box(())
        })
    });

    group.bench_function("check_packed_2x", |b| {
        b.iter(|| {
            check_packed_2x();
            black_box(())
        })
    });

    group.bench_function("check_all_packed_2x", |b| {
        b.iter(|| {
            check_all_packed_2x();
            black_box(())
        })
    });

    group.bench_function("check_all_packed_16x", |b| {
        b.iter(|| {
            check_all_packed_16x();
            black_box(())
        })
    });

    group.finish();
}

criterion_group!(benches, check_bench);
criterion_main!(benches);
