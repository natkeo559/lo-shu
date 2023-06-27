#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{CheckVector, OrderThree, Params, Permutation};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashSet, time::Duration};

fn iter_solve_3() {
    let b: HashSet<usize> = (0..OrderThree::PERMUTATIONS)
        .into_iter()
        .filter_map(|k| Permutation::<OrderThree>::kth(k).check_v())
        .map(|i| i.index)
        .collect();

    assert!(b.len() == 8)
}

fn par_iter_solve_3() {
    let b: HashSet<usize> = (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .filter_map(|k| Permutation::<OrderThree>::kth(k).check_v())
        .map(|i| i.index)
        .collect();

    assert!(b.len() == 8)
}

fn iter_dihedral_solve_3() {
    let mut result = HashSet::new();
    for k in 0..OrderThree::PERMUTATIONS {
        if let Some(p) = Permutation::<OrderThree>::kth(k).check_v() {
            result.clone_from(&p.generate_d());
            break;
        }
    }

    assert!(result.len() == 8)
}

fn par_iter_dihedral_solve_3() {
    let a = (0..OrderThree::PERMUTATIONS)
        .into_par_iter()
        .find_map_first(|i| Permutation::<OrderThree>::kth(i).check_v())
        .unwrap()
        .generate_d();

    assert!(a.len() == 8)
}

pub fn solve_order_three_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_three_solve");
    group.sample_size(800);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("iter", |b| b.iter(black_box(iter_solve_3)));
    group.bench_function("par_iter", |b| b.iter(black_box(par_iter_solve_3)));

    group.finish();
}

pub fn solve_order_three_d_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_three_solve_dihedral");
    group.sample_size(800);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("iter", |b| b.iter(black_box(iter_dihedral_solve_3)));
    group.bench_function("par_iter", |b| b.iter(black_box(par_iter_dihedral_solve_3)));

    group.finish();
}

pub fn solve_order_three_par_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_three_solve_par_iter");
    group.sample_size(800);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("default", |b| b.iter(black_box(par_iter_solve_3)));
    group.bench_function("dihedral", |b| b.iter(black_box(par_iter_dihedral_solve_3)));

    group.finish();
}

criterion_group!(
    benches,
    solve_order_three_bench,
    solve_order_three_d_bench,
    solve_order_three_par_bench,
);
criterion_main!(benches);
