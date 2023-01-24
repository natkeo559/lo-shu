use itertools::Itertools;
use crate::check::simd_double_u8_vec::simd_double_u8_vec;

pub fn iterate_perms() -> Vec<Vec<u8>> {
    (1u8..10)
        .permutations(9)
        .chunks(2)
        .into_iter()
        .filter_map(|chunk| simd_double_u8_vec(chunk.flatten().collect::<Vec<u8>>()))
        .collect::<Vec<_>>()
}
