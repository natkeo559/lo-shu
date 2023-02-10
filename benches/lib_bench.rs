use criterion::{criterion_group, criterion_main, Criterion};

use lo_shu::core::{KthPerm, Square};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn bench_this() {
    let b = Square::<u8>::first();
    let _ = (0..362890)
        .into_par_iter()
        .filter_map(|k| Some(b.clone().kth_perm(k)))
        .collect::<Vec<_>>();
}

pub fn lib_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Structured u8");
    group.bench_function("simd_double_perms_u8", |b| b.iter(|| bench_this()));

    group.finish();
}

criterion_group!(benches, lib_bench);
criterion_main!(benches);
