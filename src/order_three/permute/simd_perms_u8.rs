use itertools::Itertools;
use crate::check::simd_single_u8_vec::simd_single_u8_vec;

#[inline(never)]
pub fn iterate_perms() -> Vec<Vec<u8>> {
    (1u8..10)
        .permutations(9)
        .filter_map(simd_single_u8_vec)
        .collect::<Vec<_>>()
}
