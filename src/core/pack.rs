use crate::{Params, Permutation, Square};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackedPermutation<T: Copy + Clone, const N: usize, P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<T, P>,
    pub index: [usize; N],
}

impl<T: Copy + Clone, const N: usize, P: Params> PackedPermutation<T, N, P>
where
    [(); P::ELEMENTS]:,
{
    pub fn pack_two_from_p_iter(
        mut iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u8, N, P> {
        let owned = [iter.next().unwrap(), iter.next().unwrap()];

        if iter.next().is_some() {
            panic!("Iterator is too big!")
        }

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let mut indeces = [0usize; N];
        for (index, item) in owned.iter().map(|i| i.index).enumerate() {
            indeces[index] = item;
        }

        let mut arr = [[0u8; 2]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u8; P::ELEMENTS] = arr
            .into_iter()
            .map(pack_u4x2)
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
        mut iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u16, N, P> {
        let owned = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];

        if iter.next().is_some() {
            panic!("Iterator is too big!")
        }

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let mut indeces = [0usize; N];
        for (index, item) in owned.iter().map(|i| i.index).enumerate() {
            indeces[index] = item;
        }

        let mut arr = [[0u8; 4]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u16; P::ELEMENTS] = arr
            .into_iter()
            .map(pack_u4x4)
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
        mut iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u32, N, P> {
        let owned = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];

        if iter.next().is_some() {
            panic!("Iterator is too big!")
        }

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let mut indeces = [0usize; N];
        for (index, item) in owned.iter().map(|i| i.index).enumerate() {
            indeces[index] = item;
        }

        let mut arr = [[0u8; 8]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u32; P::ELEMENTS] = arr
            .into_iter()
            .map(pack_u4x8)
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
        mut iter: impl Iterator<Item = Permutation<u8, P>>,
    ) -> PackedPermutation<u64, N, P> {
        let owned = [
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        ];

        if iter.next().is_some() {
            panic!("Iterator is too big!")
        }

        let squares = owned
            .iter()
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let mut indeces = [0usize; N];
        for (index, item) in owned.iter().map(|i| i.index).enumerate() {
            indeces[index] = item;
        }

        let mut arr = [[0u8; 16]; P::ELEMENTS];
        for (idxo, sq) in squares {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let packed_arr: [u64; P::ELEMENTS] = arr
            .into_iter()
            .map(pack_u4x16)
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

#[rustfmt::skip]
pub fn unpack_u4x2(data: u8) -> [u8; 2] {
    [
        (data >> 4) & 0x0F,
        data & 0x0F,
    ]
}

#[rustfmt::skip]
pub fn unpack_u4x4(data: u16) -> [u8; 4] {
    [
        (data >> 12) as u8 & 0x0F,
        (data >> 8 ) as u8 & 0x0F,
        (data >> 4 ) as u8 & 0x0F,
        data as u8 & 0x0F,
    ]
}

#[rustfmt::skip]
pub fn unpack_u4x8(data: u32) -> [u8; 8] {
    [
        (data >> 28) as u8 & 0x0F,
        (data >> 24) as u8 & 0x0F,
        (data >> 20) as u8 & 0x0F,
        (data >> 16) as u8 & 0x0F,
        (data >> 12) as u8 & 0x0F,
        (data >> 8 ) as u8 & 0x0F,
        (data >> 4 ) as u8 & 0x0F,
        data as u8 & 0x0F,
    ]
}

#[rustfmt::skip]
pub fn unpack_u4x16(data: u64) -> [u8; 16] {
    [
        (data >> 60) as u8 & 0x0F,
        (data >> 56) as u8 & 0x0F,
        (data >> 52) as u8 & 0x0F,
        (data >> 48) as u8 & 0x0F,
        (data >> 44) as u8 & 0x0F,
        (data >> 40) as u8 & 0x0F,
        (data >> 36) as u8 & 0x0F,
        (data >> 32) as u8 & 0x0F,
        (data >> 28) as u8 & 0x0F,
        (data >> 24) as u8 & 0x0F,
        (data >> 20) as u8 & 0x0F,
        (data >> 16) as u8 & 0x0F,
        (data >> 12) as u8 & 0x0F,
        (data >> 8 ) as u8 & 0x0F,
        (data >> 4 ) as u8 & 0x0F,
        data as u8 & 0x0F,
    ]
}

#[cfg(test)]
mod pack_test {
    use crate::core::pack::*;
    use crate::{OrderThree, Permutation};

    #[test]
    fn test_pack_2() {
        let x_r: PackedPermutation<u8, 2, OrderThree> = PackedPermutation {
            square: Square([17, 34, 51, 68, 85, 102, 119, 137, 152]),
            index: [0, 1],
        };
        let x = Permutation::<u8, OrderThree>::permutation_range(0, 2);

        let packed = PackedPermutation::<u8, 2, OrderThree>::pack_two_from_p_iter(x);

        assert_eq!(x_r, packed);
    }

    #[test]
    fn test_pack_4() {
        let x_r: PackedPermutation<u16, 4, OrderThree> = PackedPermutation {
            square: Square([4369, 8738, 13107, 17476, 21845, 26214, 30600, 35193, 39063]),
            index: [0, 1, 2, 3],
        };

        let x = Permutation::<u8, OrderThree>::permutation_range(0, 4);

        let packed = PackedPermutation::<u8, 4, OrderThree>::pack_four_from_p_iter(x);
        assert_eq!(x_r, packed);
    }
}
