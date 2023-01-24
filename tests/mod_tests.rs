#[cfg(test)]
use lo_shu::order_three::permute::*;

const FULL_SOL: [[u8; 9]; 8] = [
    [2, 7, 6, 9, 5, 1, 4, 3, 8],
    [2, 9, 4, 7, 5, 3, 6, 1, 8],
    [4, 3, 8, 9, 5, 1, 2, 7, 6],
    [4, 9, 2, 3, 5, 7, 8, 1, 6],
    [6, 1, 8, 7, 5, 3, 2, 9, 4],
    [6, 7, 2, 1, 5, 9, 8, 3, 4],
    [8, 1, 6, 3, 5, 7, 4, 9, 2],
    [8, 3, 4, 1, 5, 9, 6, 7, 2],
];

const HALF_SOL: [[u8; 9]; 4] = [
    [2, 7, 6, 9, 5, 1, 4, 3, 8],
    [2, 9, 4, 7, 5, 3, 6, 1, 8],
    [4, 9, 2, 3, 5, 7, 8, 1, 6],
    [6, 7, 2, 1, 5, 9, 8, 3, 4],
];

#[test]
fn test_simd_perms_u8() {
    assert_eq!(simd_perms_u8::iterate_perms(), FULL_SOL)
}

#[test]
fn test_simd_threaded_kth_u8() {
    assert_eq!(simd_threaded_kth_u8::iterate_perms(), FULL_SOL)
}

#[test]
fn test_simd_double_perms_u8() {
    assert_eq!(simd_double_perms_u8::iterate_perms(), HALF_SOL)
}
