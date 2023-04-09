// use std::sync::{Arc, Mutex};
// use std::thread;

// use lo_shu::{Check, NextPerm, Permutation, Square};

// #[inline]
// fn solve_order_three_linear() -> Arc<Mutex<Vec<usize>>> {
//     static THREADS: usize = 1;
//     static COMPS: usize = 362880 / THREADS;

//     let results = Arc::new(Mutex::new(vec![]));
//     let a = Arc::new(Mutex::new(Square::<u8, 9>::first()));
//     let mut handles = vec![];

//     for _ in 0..THREADS {
//         let a = Arc::clone(&a);
//         let r = Arc::clone(&results);
//         let handle = thread::spawn(move || {
//             for _ in 0..COMPS {
//                 let mut perm = a.lock().unwrap();
//                 let mut res = r.lock().unwrap();

//                 match (*perm).next_perm() {
//                     Some(p) => match p.check_simd_single() {
//                         Some(x) => res.push(x.index),
//                         None => (),
//                     },
//                     None => break,
//                 };
//             }
//         });

//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//     }

//     results
// }

// #[inline]
// fn main() {
//     assert!(solve_order_three_linear().lock().unwrap().len() == 8)
// }

// Temporary add-in
fn main() {}
