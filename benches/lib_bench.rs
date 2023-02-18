use criterion::{criterion_group, criterion_main, Criterion};

use lo_shu::core::{Check, Square};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn perm_sol() {
    let b = (0..362879)
        .into_par_iter()
        .filter_map(|k| Square::<f64>::kth_perm(k).check_simd_single())
        .collect::<Vec<_>>();

    assert!(b.len() == 8)
}

pub fn lib_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Permutative Solver");
    group.sample_size(1000);
    group.bench_function("perm_solution", |b| b.iter(|| perm_sol()));

    group.finish();
}

criterion_group!(benches, lib_bench);
criterion_main!(benches);
