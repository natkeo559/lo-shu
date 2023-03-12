use crate::{params::Params, Square};
use rayon::prelude::*;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Permutation<T: Copy + Clone, P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<T, P>,
    pub index: usize,
}

impl<P: Params> Permutation<u8, P>
where
    [(); P::ELEMENTS]:,
{
    pub fn first() -> Permutation<u8, P> {
        let mut arr: [u8; P::ELEMENTS] = [0; P::ELEMENTS];
        for (elem, val) in arr.iter_mut().zip(1..=P::ELEMENTS as u8) {
            *elem = val;
        }
        Permutation {
            square: Square(arr),
            index: 0,
        }
    }

    pub fn kth(k: usize) -> Permutation<u8, P> {
        let mut n = Permutation::<u8, P>::first();
        let mut indeces = [0; P::ELEMENTS];

        let mut divisor = 1;
        for place in 1..P::ELEMENTS + 1 {
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
        Permutation {
            square: n.square,
            index: k,
        }
    }

    pub fn next_perm(&mut self) -> Option<&mut Permutation<u8, P>> {
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

    pub fn par_permutation_range(
        start: usize,
        stop: usize,
    ) -> impl ParallelIterator<Item = Permutation<u8, P>> {
        (start..stop).into_par_iter().map(|i| Permutation::kth(i))
    }

    pub fn permutation_range(
        start: usize,
        stop: usize,
    ) -> impl Iterator<Item = Permutation<u8, P>> {
        (start..stop).map(|i| Permutation::kth(i))
    }
}

impl<T: Clone + Copy + Hash, P: Params> Hash for Permutation<T, P>
where
    [(); P::ELEMENTS]:,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state)
    }
}

impl<T: Copy + Clone + PartialEq, P: Params> PartialEq for Permutation<T, P>
where
    [(); P::ELEMENTS]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl<T: Copy + Clone + Eq, P: Params> Eq for Permutation<T, P> where [(); P::ELEMENTS]: {}

#[cfg(test)]
mod test_perms3 {
    use rayon::prelude::ParallelIterator;

    use crate::{OrderThree, Params, Permutation, Square};

    #[test]
    fn test_first() {
        let result: Permutation<u8, OrderThree> = Permutation {
            square: Square([1, 2, 3, 4, 5, 6, 7, 8, 9]),
            index: 0,
        };
        let a = Permutation::<u8, OrderThree>::first();
        assert_eq!(result, a);
    }

    #[test]
    fn test_next() {
        let result: Permutation<u8, OrderThree> = Permutation {
            square: Square([1, 2, 3, 4, 5, 6, 7, 9, 8]),
            index: 1,
        };
        let mut a = Permutation::<u8, OrderThree>::first();
        a = *a.next_perm().unwrap();
        assert_eq!(result, a);
    }

    #[test]
    fn test_kth() {
        const MAX: usize = OrderThree::PERMUTATIONS;

        let results: [Permutation<u8, OrderThree>; 4] = [
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
        let a = Permutation::<u8, OrderThree>::kth(0);
        assert_eq!(results[0], a);

        let a = Permutation::<u8, OrderThree>::kth(1);
        assert_eq!(results[1], a);

        let a = Permutation::<u8, OrderThree>::kth(MAX - 2);
        assert_eq!(results[2], a);

        let a = Permutation::<u8, OrderThree>::kth(MAX - 1);
        assert_eq!(results[3], a);
    }

    #[test]
    fn test_par_perm_iter() {
        let a = Permutation::<u8, OrderThree>::par_permutation_range(0, 8);
        let a_vec = a.collect::<Vec<Permutation<u8, OrderThree>>>();
        assert_eq!(8, a_vec.len());
        assert_eq!(7, a_vec.last().unwrap().index)
    }

    #[test]
    fn test_perm_iter() {
        let a = Permutation::<u8, OrderThree>::permutation_range(0, 8);
        let a_vec = a.collect::<Vec<Permutation<u8, OrderThree>>>();
        assert_eq!(8, a_vec.len());
        assert_eq!(7, a_vec.last().unwrap().index)
    }
}
