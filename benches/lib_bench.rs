use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use lo_shu::core::{Check, Square3};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

fn solve_order_three_linear() {
    let b = (0..362879)
        .into_par_iter()
        .filter_map(|k| Square3::<f64>::kth_perm(k).check_simd_single())
        .collect::<Vec<_>>();

    assert!(b.len() == 8)
}

pub fn order_three_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("order_three");
    group.sample_size(1000);
    group.measurement_time(Duration::new(7, 0));
    group.noise_threshold(0.03);

    group.bench_function("solve_order_three_linear", |b| {
        b.iter(|| solve_order_three_linear())
    });

    let i = black_box(362879);
    group.bench_with_input("order_three_kth_perm", &i, |b, i| {
        b.iter(|| Square3::<f64>::kth_perm(*i))
    });

    group.finish();
}

criterion_group!(benches, order_three_bench);
criterion_main!(benches);
