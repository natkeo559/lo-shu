use std::thread;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crossbeam_channel::unbounded;
use lo_shu::{Enumerable, Permutation, O3};

fn crossbeam_solver_o3(t: usize) {
    let (s, r) = unbounded();

    for i in 0..t {
        let sender = s.clone();
        thread::spawn(move || {
            for n in (i..69075).step_by(t) {
                if let Some(sol) = Permutation::<O3>::kth(n.try_into().unwrap()).check_n_s() {
                    match sender.send(sol) {
                        Ok(_) => {}
                        Err(_) => {}
                    };
                }
            }
        });
    }

    let mut sols = Vec::with_capacity(8);
    loop {
        match r.recv() {
            Ok(p) => sols.push(p),
            Err(_) => {
                println!("ERR recv")
            }
        }

        if sols.len() == 1 {
            break;
        }
    }
}

pub fn crossbeam_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("crossbeam");
    group.sample_size(3000);
    group.noise_threshold(0.03);

    let input = [
        (1, "order_three_1"),
        (2, "order_three_2"),
        (4, "order_three_4"),
        (8, "order_three_8"),
        (16, "order_three_16"),
        (32, "order_three_32"),
        (64, "order_three_64"),
    ];
    for inp in input {
        group.bench_with_input(inp.1, &inp, |b, i| {
            b.iter(|| crossbeam_solver_o3(black_box(i.0)))
        });
    }

    group.finish();
}

criterion_group!(benches, crossbeam_bench);
criterion_main!(benches);
