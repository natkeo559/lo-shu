use lo_shu::{Check, Square3};
use rayon::prelude::*;

fn perm_sol() {
    let _ = (0..362879 / 2)
        .into_par_iter()
        .filter_map(|k| Square3::<u8>::kth_perm(k).check_simd_single())
        .collect::<Vec<_>>();
}

fn main() {
    perm_sol();
}
