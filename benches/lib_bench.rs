#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lo_shu::{Check, OrderThree, Permutation};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::time::Duration;

fn solve_order_three_linear() {
    let b = (0..362879)
        .into_par_iter()
        .filter_map(|k| unsafe { Permutation::<u8, OrderThree>::kth(k).unsafe_check_strict() })
        .map(|r| r.index)
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

    group.bench_function("kth", |b| {
        b.iter(|| Permutation::<u8, OrderThree>::kth(black_box(100)))
    });

    group.finish();
}

criterion_group!(benches, order_three_bench);
criterion_main!(benches);

// #![feature(generic_const_exprs)]
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

// use criterion::{black_box, criterion_group, criterion_main, Criterion};

// use lo_shu::{NextPerm, Permutation, Square, OrderThree};
// use rayon::iter::{IntoParallelIterator, ParallelIterator};

// fn solve_order_three_linear() {
//     let results = Arc::new(Mutex::new(vec![]));
//     let a = Arc::new(Mutex::new(Permutation::<u8, OrderThree>::first()));
//     let mut handles = vec![];

//     for _ in 0..16 {
//         let a = Arc::clone(&a);
//         let r = Arc::clone(&results);
//         let handle = thread::spawn(move || {
//             for _ in 0..22680 {
//                 let mut perm = a.lock().unwrap();
//                 let mut res = r.lock().unwrap();

//                 if (*perm).index == 362880 {
//                     res.push(*perm);
//                     break;
//                 }
//                 res.push(*perm);
//                 *perm = *perm.next_perm().unwrap();

//                 // *perm = *perm.next_perm().unwrap_or(&mut Square::<u8, 9>::first());
//             }
//         });

//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     assert!(results.lock().unwrap().len() == 362880);
// }

// pub fn order_three_bench(c: &mut Criterion) {
//     let mut group = c.benchmark_group("order_three");
//     group.sample_size(10);
//     group.measurement_time(Duration::new(7, 0));
//     group.noise_threshold(0.03);

//     group.bench_function("solve_order_three_linear", |b| {
//         b.iter(|| solve_order_three_linear())
//     });

//     group.finish();
// }

// criterion_group!(benches, order_three_bench);
// criterion_main!(benches);
