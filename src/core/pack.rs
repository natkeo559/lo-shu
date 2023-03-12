use crate::{Params, Square};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PackedPermutation<T: Copy + Clone, P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<T, P>,
    pub index: Vec<usize>,
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
        let x = Permutation::<u8, OrderThree>::permutation_range(0, 2)
            .map(|i| i.square.0.into_iter().enumerate())
            .enumerate();

        let mut arr = [[0u8; 2]; 9];
        for (idxo, sq) in x {
            for (idxs, i) in sq {
                arr[idxs][idxo] = i
            }
        }

        let _packed_arr: [u8; 9] = arr
            .into_iter()
            .map(|i| pack_u4x2(i))
            .collect::<Vec<_>>()
            .as_slice()
            .try_into()
            .unwrap();
    }
}
