use std::time::Duration;

use criterion::{criterion_group, criterion_main, Criterion};

use lo_shu::core::{Check, Square3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn solve_order_three_linear() {
    let b = (0..362879)
        .into_par_iter()
        .filter_map(|k| Square3::<f64>::kth_perm(k).check_simd_single())
        .collect::<Vec<_>>();

    assert!(b.len() == 8)
}

fn kth_perm_bench() {
    let _ = Square3::<f64>::kth_perm(362879);
}

pub fn order_three_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_three");
    group.sample_size(1000);
    group.measurement_time(Duration::new(7, 0));
    group.noise_threshold(0.03);

    group.bench_function("solve_order_three_linear", |b| {
        b.iter(|| solve_order_three_linear())
    });

    group.bench_function("order_three_kth_perm", |b| b.iter(|| kth_perm_bench()));

    group.finish();
}

criterion_group!(benches, order_three_bench);
criterion_main!(benches);
