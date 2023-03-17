use itertools::Itertools;

use crate::{Params, Permutation, Square};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackedPermutation<T: Copy + Clone, P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<T, P>,
    pub index: Vec<usize>,
}

impl<T: Copy + Clone, P: Params> PackedPermutation<T, P>
where
    [(); P::ELEMENTS]:,
{
    pub fn pack_two_from_p_iter(
        iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u8, P> {
        let owned = iter.collect_vec();

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let indeces = owned.iter().map(|i| i.index).collect_vec();

        let mut arr = [[0u8; 2]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u8; P::ELEMENTS] = arr
            .into_iter()
            .map(|i| pack_u4x2(i))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        PackedPermutation {
            square: Square(packed_arr),
            index: indeces,
        }
    }

    pub fn pack_four_from_p_iter(
        iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u16, P> {
        let owned = iter.collect_vec();

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let indeces = owned.iter().map(|i| i.index).collect_vec();

        let mut arr = [[0u8; 4]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u16; P::ELEMENTS] = arr
            .into_iter()
            .map(|i| pack_u4x4(i))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        PackedPermutation {
            square: Square(packed_arr),
            index: indeces,
        }
    }

    pub fn pack_eight_from_p_iter(
        iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u32, P> {
        let owned = iter.collect_vec();

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let indeces = owned.iter().map(|i| i.index).collect_vec();

        let mut arr = [[0u8; 8]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u32; P::ELEMENTS] = arr
            .into_iter()
            .map(|i| pack_u4x8(i))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        PackedPermutation {
            square: Square(packed_arr),
            index: indeces,
        }
    }

    pub fn pack_sixteen_from_p_iter(
        iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u64, P> {
        let owned = iter.collect_vec();

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let indeces = owned.iter().map(|i| i.index).collect_vec();

        let mut arr = [[0u8; 16]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u64; P::ELEMENTS] = arr
            .into_iter()
            .map(|i| pack_u4x16(i))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();

        PackedPermutation {
            square: Square(packed_arr),
            index: indeces,
        }
    }
}

#[rustfmt::skip]
pub fn pack_u4x2(data: [u8; 2]) -> u8 {
    ((data[0] & 0x0F) << 4) | (data[1] & 0x0F)
}

#[rustfmt::skip]
pub fn pack_u4x4(data: [u8; 4]) -> u16 {
    (data[3] as u16 & 0x0F)
        | (data[2] as u16 & 0x0F) << 4
        | (data[1] as u16 & 0x0F) << 8
        | (data[0] as u16 & 0x0F) << 12
}

#[rustfmt::skip]
pub fn pack_u4x8(data: [u8; 8]) -> u32 {
    (data[7] as u32 & 0x0F)
        | (data[6] as u32 & 0x0F) << 4
        | (data[5] as u32 & 0x0F) << 8
        | (data[4] as u32 & 0x0F) << 12
        | (data[3] as u32 & 0x0F) << 16
        | (data[2] as u32 & 0x0F) << 20
        | (data[1] as u32 & 0x0F) << 24
        | (data[0] as u32 & 0x0F) << 28
}

#[rustfmt::skip]
pub fn pack_u4x16(data: [u8; 16]) -> u64 {
    (data[15] as u64 & 0x0F)
        | (data[14] as u64 & 0x0F) << 4
        | (data[13] as u64 & 0x0F) << 8
        | (data[12] as u64 & 0x0F) << 12
        | (data[11] as u64 & 0x0F) << 16
        | (data[10] as u64 & 0x0F) << 20
        | (data[9 ] as u64 & 0x0F) << 24
        | (data[8 ] as u64 & 0x0F) << 28
        | (data[7 ] as u64 & 0x0F) << 32
        | (data[6 ] as u64 & 0x0F) << 36
        | (data[5 ] as u64 & 0x0F) << 40
        | (data[4 ] as u64 & 0x0F) << 44
        | (data[3 ] as u64 & 0x0F) << 48
        | (data[2 ] as u64 & 0x0F) << 52
        | (data[1 ] as u64 & 0x0F) << 56
        | (data[0 ] as u64 & 0x0F) << 60
}

#[cfg(test)]
mod pack_test {
    use crate::core::pack::*;
    use crate::{OrderThree, Permutation};

    #[test]
    fn test_pack_2() {
        let x_r: PackedPermutation<u8, OrderThree> = PackedPermutation {
            square: Square([17, 34, 51, 68, 85, 102, 119, 137, 152]),
            index: vec![0, 1],
        };
        let x = Permutation::<u8, OrderThree>::permutation_range(0, 2);

        let packed = PackedPermutation::<u8, OrderThree>::pack_two_from_p_iter(x);

        assert_eq!(x_r, packed);
    }
}
