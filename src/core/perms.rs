use rayon::prelude::*;

use crate::Square3;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Permutation<T: Copy + Clone> {
    pub square: Square3<T>,
    pub index: usize,
}

macro_rules! impl_int_perms {
    ($t: ty) => {
        impl Square3<$t> {
            /// Constructor for a permutation given k, the index of the permutation from its lexcographically ordered permutation group, Sym(M).
            ///
            /// Based on:
            /// https://stackoverflow.com/questions/31216097/given-n-and-k-return-the-kth-permutation-sequence
            ///
            pub fn kth_perm(k: i32) -> Permutation<$t> {
                let mut n = [1, 2, 3, 4, 5, 6, 7, 8, 9];
                let mut indeces = [0; 9];

                let mut divisor = 1;
                for place in 1..10 {
                    if k / divisor == 0 {
                        break;
                    }
                    indeces[9 - place] = (k / divisor) % place as i32;
                    divisor *= place as i32;
                }
                for i in 0..9 {
                    let index = indeces[i] as usize + i;
                    if index != i {
                        let temp = n[index];
                        let mut j = index;
                        while j > i {
                            n[j] = n[j - 1];
                            j -= 1;
                        }
                        n[i] = temp;
                    }
                }
                Permutation {
                    square: Square3 { array: n },
                    index: k as usize,
                }
            }
        }
    };
}

macro_rules! impl_float_perms {
    ($t: ty) => {
        impl Square3<$t> {
            /// Constructor for a permutation given k, the index of the permutation from its lexcographically ordered permutation group, Sym(M).
            //
            /// Based on:
            /// https://stackoverflow.com/questions/31216097/given-n-and-k-return-the-kth-permutation-sequence
            ///
            pub fn kth_perm(k: i32) -> Permutation<$t> {
                let mut n = [1., 2., 3., 4., 5., 6., 7., 8., 9.];
                let mut indeces = [0; 9];

                let mut divisor = 1;
                for place in 1..10 {
                    if k / divisor == 0 {
                        break;
                    }
                    indeces[9 - place] = (k / divisor) % place as i32;
                    divisor *= place as i32;
                }
                for i in 0..9 {
                    let index = indeces[i] as usize + i;
                    if index != i {
                        let temp = n[index];
                        let mut j = index;
                        while j > i {
                            n[j] = n[j - 1];
                            j -= 1;
                        }
                        n[i] = temp;
                    }
                }
                Permutation {
                    square: Square3 { array: n },
                    index: k as usize,
                }
            }
        }
    };
}

macro_rules! impl_perm_range {
    ($t: ty) => {
        impl Square3<$t> {
            /// Produces an iterator of permutations given the start and end index of the desired permutations from its lexcographically ordered permutation group, Sym(M)
            ///
            pub fn permutation_range(
                start: usize,
                stop: usize,
            ) -> impl ParallelIterator<Item = Permutation<$t>> {
                (start..stop)
                    .into_par_iter()
                    .map(|i| Square3::<$t>::kth_perm(i as i32))
            }
        }
    };
}

impl_int_perms!(u8);
impl_int_perms!(u16);

impl_float_perms!(f32);
impl_float_perms!(f64);

impl_perm_range!(u8);
impl_perm_range!(u16);
impl_perm_range!(f32);
impl_perm_range!(f64);

#[cfg(test)]
mod test_perms3 {
    use super::*;

    #[test]
    fn test_kth() {
        let a: Permutation<u8> = Square3::<u8>::kth_perm(362879);
        let a_result = Permutation {
            square: Square3 {
                array: [9u8, 8, 7, 6, 5, 4, 3, 2, 1],
            },
            index: 362879,
        };

        assert_eq!(a, a_result);

        let a: Permutation<f32> = Square3::<f32>::kth_perm(362879);
        let a_result = Permutation {
            square: Square3 {
                array: [9., 8., 7., 6., 5., 4., 3., 2., 1.],
            },
            index: 362879,
        };

        assert_eq!(a, a_result);
    }
}
