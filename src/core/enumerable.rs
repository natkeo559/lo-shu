use crate::{Params, Permutation, O3, O4, O5};

pub trait EnumerableMarker {}

impl EnumerableMarker for O3 {}
impl EnumerableMarker for O4 {}
impl EnumerableMarker for O5 {}

pub trait IndexConst<T> {
    const MAX_INDEX: T;
}

macro_rules! impl_consts_for_enumerable_params {
    ($p:tt, $t:ty, $l:literal) => {
        impl IndexConst<$t> for $p {
            const MAX_INDEX: $t = $l;
        }
    };
}

impl_consts_for_enumerable_params!(O3, u32, 362880);
impl_consts_for_enumerable_params!(O4, u64, 20922789888000);
impl_consts_for_enumerable_params!(O5, u128, 15511210043330985984000000);

pub trait Enumerable<P: Params + EnumerableMarker, T>
where
    [(); P::ELEMENTS]:,
{
    fn kth(k: T) -> Permutation<P>;
    fn index(&self) -> T;
}

macro_rules! impl_fns_for_enumerable_params {
    ($p:tt, $t:ty) => {
        impl Enumerable<$p, $t> for Permutation<$p> {
            fn kth(k: $t) -> Self {
                let mut n = Self::identity();
                let mut indeces = [0; $p::ELEMENTS];

                let mut divisor = 1;
                for place in 1..=$p::ELEMENTS as $t {
                    if k / divisor == 0 {
                        break;
                    }
                    indeces[$p::ELEMENTS - place as usize] = (k / divisor) % place;
                    divisor *= place;
                }
                for (i, item) in indeces.iter().enumerate() {
                    let index = item + i as $t;
                    if index != i as $t {
                        let temp = n.square.data[index as usize];
                        let mut j = index as usize;
                        while j > i {
                            n.square.data[j] = n.square.data[j - 1];
                            j -= 1;
                        }
                        n.square.data[i] = temp;
                    }
                }
                n
            }

            fn index(&self) -> $t {
                let mut index = 0;
                let mut position = 2;
                let mut factor = 1;
                for p in (0..=$p::ELEMENTS - 2).rev() {
                    let mut succ = 0;
                    for q in p + 1..$p::ELEMENTS {
                        if self.square.data[p] > self.square.data[q] {
                            succ += 1;
                        }
                    }
                    index += succ * factor;
                    factor *= position;
                    position += 1;
                }

                index
            }
        }
    };
}

impl_fns_for_enumerable_params!(O3, u32);
impl_fns_for_enumerable_params!(O4, u64);
impl_fns_for_enumerable_params!(O5, u128);

#[cfg(test)]
mod test_enumerable {
    use crate::{Enumerable, IndexConst, ParameterSetError, Permutation, O3, O4, O5};

    #[test]
    fn test_constants() {
        assert_eq!(O3::MAX_INDEX, 362880);
        assert_eq!(O4::MAX_INDEX, 20922789888000);
        assert_eq!(O5::MAX_INDEX, 15511210043330985984000000);
    }

    #[test]
    fn test_kth_3() -> Result<(), ParameterSetError> {
        let a_result = Permutation::<O3>::try_from([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice())?;
        let b_result = Permutation::<O3>::try_from([1, 2, 3, 4, 5, 6, 7, 9, 8].as_slice())?;
        let c_result = Permutation::<O3>::try_from([9, 8, 7, 6, 5, 4, 3, 1, 2].as_slice())?;
        let d_result = Permutation::<O3>::try_from([9, 8, 7, 6, 5, 4, 3, 2, 1].as_slice())?;

        let a = Permutation::<O3>::kth(0);
        let b = Permutation::<O3>::kth(1);
        let c = Permutation::<O3>::kth(362878);
        let d = Permutation::<O3>::kth(362879);

        assert_eq!(a, a_result);
        assert_eq!(b, b_result);
        assert_eq!(c, c_result);
        assert_eq!(d, d_result);

        Ok(())
    }

    #[test]
    fn test_index_3() -> Result<(), ParameterSetError> {
        let a = Permutation::<O3>::try_from([1, 2, 3, 4, 5, 6, 7, 8, 9].as_slice())?.index();
        let b = Permutation::<O3>::try_from([1, 2, 3, 4, 5, 6, 7, 9, 8].as_slice())?.index();
        let c = Permutation::<O3>::try_from([9, 8, 7, 6, 5, 4, 3, 1, 2].as_slice())?.index();
        let d = Permutation::<O3>::try_from([9, 8, 7, 6, 5, 4, 3, 2, 1].as_slice())?.index();

        let a_result = 0;
        let b_result = 1;
        let c_result = 362878;
        let d_result = 362879;

        assert_eq!(a, a_result);
        assert_eq!(b, b_result);
        assert_eq!(c, c_result);
        assert_eq!(d, d_result);

        Ok(())
    }

    #[test]
    fn test_kth_4() -> Result<(), ParameterSetError> {
        let a_result = Permutation::<O4>::try_from(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].as_slice(),
        )?;
        let b_result = Permutation::<O4>::try_from(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 15].as_slice(),
        )?;
        let c_result = Permutation::<O4>::try_from(
            [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 1, 2].as_slice(),
        )?;
        let d_result = Permutation::<O4>::try_from(
            [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].as_slice(),
        )?;

        let a = Permutation::<O4>::kth(0);
        let b = Permutation::<O4>::kth(1);
        let c = Permutation::<O4>::kth(20922789887998);
        let d = Permutation::<O4>::kth(20922789887999);

        assert_eq!(a, a_result);
        assert_eq!(b, b_result);
        assert_eq!(c, c_result);
        assert_eq!(d, d_result);

        Ok(())
    }

    #[test]
    fn test_index_4() -> Result<(), ParameterSetError> {
        let a = Permutation::<O4>::try_from(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].as_slice(),
        )?
        .index();
        let b = Permutation::<O4>::try_from(
            [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 15].as_slice(),
        )?
        .index();
        let c = Permutation::<O4>::try_from(
            [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 1, 2].as_slice(),
        )?
        .index();
        let d = Permutation::<O4>::try_from(
            [16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1].as_slice(),
        )?
        .index();

        let a_result = 0;
        let b_result = 1;
        let c_result = 20922789887998;
        let d_result = 20922789887999;

        assert_eq!(a, a_result);
        assert_eq!(b, b_result);
        assert_eq!(c, c_result);
        assert_eq!(d, d_result);

        Ok(())
    }

    #[test]
    fn test_kth_5() -> Result<(), ParameterSetError> {
        let a_result = Permutation::<O5>::try_from(
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ]
            .as_slice(),
        )?;
        let b_result = Permutation::<O5>::try_from(
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                25, 24,
            ]
            .as_slice(),
        )?;
        let c_result = Permutation::<O5>::try_from(
            [
                25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
                3, 1, 2,
            ]
            .as_slice(),
        )?;
        let d_result = Permutation::<O5>::try_from(
            [
                25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
                3, 2, 1,
            ]
            .as_slice(),
        )?;

        let a = Permutation::<O5>::kth(0);
        let b = Permutation::<O5>::kth(1);
        let c = Permutation::<O5>::kth(15511210043330985983999998);
        let d = Permutation::<O5>::kth(15511210043330985983999999);

        assert_eq!(a, a_result);
        assert_eq!(b, b_result);
        assert_eq!(c, c_result);
        assert_eq!(d, d_result);

        Ok(())
    }

    #[test]
    fn test_index_5() -> Result<(), ParameterSetError> {
        let a = Permutation::<O5>::try_from(
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25,
            ]
            .as_slice(),
        )?
        .index();
        let b = Permutation::<O5>::try_from(
            [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                25, 24,
            ]
            .as_slice(),
        )?
        .index();
        let c = Permutation::<O5>::try_from(
            [
                25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
                3, 1, 2,
            ]
            .as_slice(),
        )?
        .index();
        let d = Permutation::<O5>::try_from(
            [
                25, 24, 23, 22, 21, 20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4,
                3, 2, 1,
            ]
            .as_slice(),
        )?
        .index();

        let a_result = 0;
        let b_result = 1;
        let c_result = 15511210043330985983999998;
        let d_result = 15511210043330985983999999;

        assert_eq!(a, a_result);
        assert_eq!(b, b_result);
        assert_eq!(c, c_result);
        assert_eq!(d, d_result);

        Ok(())
    }
}
