#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![feature(portable_simd)]
#![feature(iter_array_chunks)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{CheckVector, OrderThree, Params, Permutation};
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};
use std::simd::{u8x64, u8x8, Simd};
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

fn wide_simd_3() {
    let perm_arr = (0..OrderThree::PERMUTATIONS)
        .map(|k| Permutation::<OrderThree>::kth(k).square)
        .array_chunks::<8>();
    const VMASK: Simd<u8, 8> = u8x8::from_slice(&[15; 8]);

    let mut valid = vec![];
    for i in perm_arr {
        let mut elems_0 = [0; 64];
        let mut elems_1 = [0; 64];
        let mut elems_2 = [0; 64];

        for n in 0..i.len() {
            elems_0[0 + (8 * n)] = i[n][0];
            elems_0[1 + (8 * n)] = i[n][1];
            elems_0[2 + (8 * n)] = i[n][2];
            elems_0[3 + (8 * n)] = i[n][0];
            elems_0[4 + (8 * n)] = i[n][3];
            elems_0[5 + (8 * n)] = i[n][6];
            elems_0[6 + (8 * n)] = i[n][0];
            elems_0[7 + (8 * n)] = i[n][2];

            elems_1[0 + (8 * n)] = i[n][3];
            elems_1[1 + (8 * n)] = i[n][4];
            elems_1[2 + (8 * n)] = i[n][5];
            elems_1[3 + (8 * n)] = i[n][1];
            elems_1[4 + (8 * n)] = i[n][4];
            elems_1[5 + (8 * n)] = i[n][7];
            elems_1[6 + (8 * n)] = i[n][4];
            elems_1[7 + (8 * n)] = i[n][4];

            elems_2[0 + (8 * n)] = i[n][6];
            elems_2[1 + (8 * n)] = i[n][7];
            elems_2[2 + (8 * n)] = i[n][8];
            elems_2[3 + (8 * n)] = i[n][2];
            elems_2[4 + (8 * n)] = i[n][5];
            elems_2[5 + (8 * n)] = i[n][8];
            elems_2[6 + (8 * n)] = i[n][8];
            elems_2[7 + (8 * n)] = i[n][6];
        }
        let sums_0 = u8x64::from_slice(&elems_0);
        let sums_1 = u8x64::from_slice(&elems_1);
        let sums_2 = u8x64::from_slice(&elems_2);

        let res = sums_0 + sums_1 + sums_2;
        for (idx, r) in res
            .to_array()
            .chunks_exact(8)
            .map(|a| u8x8::from_slice(a))
            .enumerate()
        {
            if r == VMASK {
                valid.push(i[idx])
            }
        }
    }

    assert!(valid.len() == 8)
}

fn wide_simd_threaded_3() {
    let perm_arr = (0..OrderThree::PERMUTATIONS)
        .map(|k| Permutation::<OrderThree>::kth(k).square)
        .array_chunks::<8>();
    const VMASK: Simd<u8, 8> = u8x8::from_slice(&[15; 8]);

    let result: Vec<_> = perm_arr
        .par_bridge()
        .filter_map(|i| {
            let mut elems_0 = [0; 64];
            let mut elems_1 = [0; 64];
            let mut elems_2 = [0; 64];

            for n in 0..i.len() {
                elems_0[0 + (8 * n)] = i[n][0];
                elems_0[1 + (8 * n)] = i[n][1];
                elems_0[2 + (8 * n)] = i[n][2];
                elems_0[3 + (8 * n)] = i[n][0];
                elems_0[4 + (8 * n)] = i[n][3];
                elems_0[5 + (8 * n)] = i[n][6];
                elems_0[6 + (8 * n)] = i[n][0];
                elems_0[7 + (8 * n)] = i[n][2];

                elems_1[0 + (8 * n)] = i[n][3];
                elems_1[1 + (8 * n)] = i[n][4];
                elems_1[2 + (8 * n)] = i[n][5];
                elems_1[3 + (8 * n)] = i[n][1];
                elems_1[4 + (8 * n)] = i[n][4];
                elems_1[5 + (8 * n)] = i[n][7];
                elems_1[6 + (8 * n)] = i[n][4];
                elems_1[7 + (8 * n)] = i[n][4];

                elems_2[0 + (8 * n)] = i[n][6];
                elems_2[1 + (8 * n)] = i[n][7];
                elems_2[2 + (8 * n)] = i[n][8];
                elems_2[3 + (8 * n)] = i[n][2];
                elems_2[4 + (8 * n)] = i[n][5];
                elems_2[5 + (8 * n)] = i[n][8];
                elems_2[6 + (8 * n)] = i[n][8];
                elems_2[7 + (8 * n)] = i[n][6];
            }
            let sums_0 = u8x64::from_slice(&elems_0);
            let sums_1 = u8x64::from_slice(&elems_1);
            let sums_2 = u8x64::from_slice(&elems_2);

            let res = sums_0 + sums_1 + sums_2;
            let mut valid = vec![];
            for (idx, r) in res
                .to_array()
                .chunks_exact(8)
                .map(|a| u8x8::from_slice(a))
                .enumerate()
            {
                if r == VMASK {
                    valid.push(i[idx])
                }
            }
            if !valid.is_empty() {
                Some(valid)
            } else {
                None
            }
        })
        .collect();

    assert_eq!(result.len(), 8)
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

pub fn solve_order_three_wide_simd_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide_simd");
    group.sample_size(800);
    group.noise_threshold(0.03);
    group.measurement_time(Duration::new(20, 0));
    group.bench_function("single_threaded", |b| b.iter(black_box(wide_simd_3)));
    group.bench_function("multi_threaded", |b| {
        b.iter(black_box(wide_simd_threaded_3))
    });
    group.finish();
}

criterion_group!(
    benches,
    solve_order_three_bench,
    solve_order_three_d_bench,
    solve_order_three_par_bench,
    solve_order_three_wide_simd_bench,
);
criterion_main!(benches);
