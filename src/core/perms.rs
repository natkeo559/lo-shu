use crate::{order::Params, Square};
use rayon::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Permutation<P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<P>,
    pub index: usize,
}

impl<P: Params> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn identity() -> Self {
        let mut arr: [u8; P::ELEMENTS] = [0; P::ELEMENTS];
        for (elem, val) in arr.iter_mut().zip(1..=P::ELEMENTS as u8) {
            *elem = val;
        }
        Self {
            square: Square(arr),
            index: 0,
        }
    }

    pub fn kth(k: usize) -> Self {
        let mut n = Self::identity();
        let mut indeces = [0; P::ELEMENTS];

        let mut divisor = 1;
        for place in 1..=P::ELEMENTS {
            if k / divisor == 0 {
                break;
            }
            indeces[P::ELEMENTS - place] = (k / divisor) % place;
            divisor *= place;
        }
        for (i, item) in indeces.iter().enumerate() {
            let index = item + i;
            if index != i {
                let temp = n.square[index];
                let mut j = index;
                while j > i {
                    n.square[j] = n.square[j - 1];
                    j -= 1;
                }
                n.square[i] = temp;
            }
        }
        n.index = k;
        n
    }

    pub fn next_perm(&mut self) -> Option<&mut Self> {
        // Find non-increasing suffix
        let mut i: usize = P::ELEMENTS - 1;
        while i > 0 && self.square[i - 1] >= self.square[i] {
            i -= 1;
        }
        if i == 0 {
            return None;
        }

        // Find successor to pivot
        let mut j: usize = P::ELEMENTS - 1;
        while self.square[j] <= self.square[i - 1] {
            j -= 1;
        }
        self.square.swap(i - 1, j);

        // Reverse suffix
        self.square[i..].reverse();
        self.index += 1;
        Some(self)
    }

    pub fn par_permutation_range(start: usize, stop: usize) -> impl ParallelIterator<Item = Self> {
        (start..stop).into_par_iter().map(|i| Self::kth(i))
    }

    pub fn permutation_range(start: usize, stop: usize) -> impl Iterator<Item = Self> {
        (start..stop).map(|i| Self::kth(i))
    }
}

impl<P: Params> Hash for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<P: Params> PartialEq for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<P: Params> Eq for Permutation<P> where [(); P::ELEMENTS]: {}

#[cfg(test)]
mod test_perms {
    use rayon::prelude::ParallelIterator;

    use crate::{OrderFour, OrderThree, Params, Permutation, Square};

    #[test]
    fn test_first_3() {
        let result: Permutation<OrderThree> = Permutation {
            square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9]),
            index: 0,
        };
        let a = Permutation::<OrderThree>::identity();
        assert_eq!(result, a);
    }

    #[test]
    fn trya() {
        let k = 10;
        let a = (1..=5)
            .map(|b| (1..b).fold(1, |a, b| a * b))
            .filter(|a| k / a != 0)
            .enumerate();

        for i in a {
            println!("{:?}", i);
        }
    }

    #[test]
    fn test_first_4() {
        let result: Permutation<OrderFour> = Permutation {
            square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
            index: 0,
        };
        let a = Permutation::<OrderFour>::identity();
        assert_eq!(result, a);
    }

    #[test]
    fn test_next_3() {
        let result: Permutation<OrderThree> = Permutation {
            square: Square([1, 2, 3, 4, 5, 6, 7, 9, 8]),
            index: 1,
        };
        let mut a = Permutation::<OrderThree>::identity();
        a = *a.next_perm().unwrap();
        assert_eq!(result, a);
    }

    #[test]
    fn test_next_4() {
        let result: Permutation<OrderFour> = Permutation {
            square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 15]),
            index: 1,
        };
        let mut a = Permutation::<OrderFour>::identity();
        a = *a.next_perm().unwrap();
        assert_eq!(result, a);
    }

    #[test]
    fn test_kth_3() {
        const MAX: usize = OrderThree::PERMUTATIONS;

        let results: [Permutation<OrderThree>; 4] = [
            Permutation {
                square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9]),
                index: 0,
            },
            Permutation {
                square: Square([1, 2, 3, 4, 5, 6, 7, 9, 8]),
                index: 1,
            },
            Permutation {
                square: Square([9, 8, 7, 6, 5, 4, 3, 1, 2]),
                index: 362878,
            },
            Permutation {
                square: Square([9, 8, 7, 6, 5, 4, 3, 2, 1]),
                index: 362879,
            },
        ];
        let a = Permutation::<OrderThree>::kth(0);
        assert_eq!(results[0], a);

        let a = Permutation::<OrderThree>::kth(1);
        assert_eq!(results[1], a);

        let a = Permutation::<OrderThree>::kth(MAX - 2);
        assert_eq!(results[2], a);

        let a = Permutation::<OrderThree>::kth(MAX - 1);
        assert_eq!(results[3], a);
    }

    #[test]
    fn test_kth_4() {
        const MAX: usize = OrderFour::PERMUTATIONS;

        let results: [Permutation<OrderFour>; 4] = [
            Permutation {
                square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]),
                index: 0,
            },
            Permutation {
                square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 15]),
                index: 1,
            },
            Permutation {
                square: Square([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 1, 2]),
                index: 20922789887998,
            },
            Permutation {
                square: Square([16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]),
                index: 20922789887999,
            },
        ];
        let a = Permutation::<OrderFour>::kth(0);
        assert_eq!(results[0], a);

        let a = Permutation::<OrderFour>::kth(1);
        assert_eq!(results[1], a);

        let a = Permutation::<OrderFour>::kth(MAX - 2);
        assert_eq!(results[2], a);

        let a = Permutation::<OrderFour>::kth(MAX - 1);
        assert_eq!(results[3], a);
    }

    #[test]
    fn test_par_perm_iter_3() {
        let a = Permutation::<OrderThree>::par_permutation_range(0, 8);
        let a_vec = a.collect::<Vec<Permutation<OrderThree>>>();
        assert_eq!(8, a_vec.len());
        assert_eq!(7, a_vec.last().unwrap().index)
    }

    #[test]
    fn test_par_perm_iter_4() {
        let a = Permutation::<OrderFour>::par_permutation_range(0, 8);
        let a_vec = a.collect::<Vec<Permutation<OrderFour>>>();
        assert_eq!(8, a_vec.len());
        assert_eq!(7, a_vec.last().unwrap().index)
    }

    #[test]
    fn test_perm_iter_3() {
        let a = Permutation::<OrderThree>::permutation_range(0, 8);
        let a_vec = a.collect::<Vec<Permutation<OrderThree>>>();
        assert_eq!(8, a_vec.len());
        assert_eq!(7, a_vec.last().unwrap().index)
    }

    #[test]
    fn test_perm_iter_4() {
        let a = Permutation::<OrderFour>::permutation_range(0, 8);
        let a_vec = a.collect::<Vec<Permutation<OrderFour>>>();
        assert_eq!(8, a_vec.len());
        assert_eq!(7, a_vec.last().unwrap().index)
    }
}
