use crate::ParameterSetError;
use crate::{order::Params, Square};
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Copy, Debug, PartialOrd)]
pub struct Permutation<P: Params>
where
    [(); P::ELEMENTS]:,
{
    pub square: Square<P>,
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn identity() -> Self {
        let mut data: [u32; P::ELEMENTS] = [0; P::ELEMENTS];
        for (elem, val) in data.iter_mut().zip(1..=P::ELEMENTS as u32) {
            *elem = val;
        }
        Self {
            square: Square { data },
        }
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
        Some(self)
    }

    // pub fn random_index(&self) -> Self {
    //     let range = Uniform::new_inclusive(0, P::PERMUTATIONS);
    //     let mut rng = rand::thread_rng();
    //     let sample = range.sample(&mut rng);
    //     Permutation::kth(sample)
    // }

    // pub fn random_inplace(&mut self) {
    //     let mut rng = rand::thread_rng();
    //     self.square.0.shuffle(&mut rng);
    //     self.index = self.square.to_perm().index;
    // }

    // pub fn rand_index_samples(n: usize) -> Vec<usize> {
    //     let range = Uniform::new_inclusive(0, P::PERMUTATIONS);
    //     let mut rng = rand::thread_rng();
    //     range.sample_iter(&mut rng).take(n).collect()
    // }

    // pub fn par_permutation_range(start: usize, stop: usize) -> impl ParallelIterator<Item = Self> {
    //     (start..stop).into_par_iter().map(|i| Self::kth(i))
    // }

    // pub fn permutation_range(start: usize, stop: usize) -> impl Iterator<Item = Self> {
    //     (start..stop).map(|i| Self::kth(i))
    // }

    // pub fn indexes(arr: &[usize]) -> impl Iterator<Item = Self> + '_ {
    //     arr.iter().map(|&k| Self::kth(k))
    // }
}

impl<P: Params> Hash for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.square.hash(state)
    }
}

impl<P: Params> PartialEq for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn eq(&self, other: &Self) -> bool {
        self.square == other.square
    }
}

impl<P: Params> Eq for Permutation<P> where [(); P::ELEMENTS]: {}

impl<P: Params> fmt::Display for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.square)?;

        Ok(())
    }
}

impl<P: Params> TryFrom<&[u32]> for Permutation<P> where [(); P::ELEMENTS]: {
    type Error = ParameterSetError;

    fn try_from(slice: &[u32]) -> Result<Self, Self::Error> {
        let s = Square::<P>::try_from(slice)?;
        Ok(Permutation { square: s })
    }
}

#[cfg(test)]
mod test_perms {

    use crate::{O4, O3, Permutation, Square};

    #[test]
    fn test_first_3() {
        let result: Permutation<O3> = Permutation {
            square: Square { data: [1, 2, 3, 4, 5, 6, 7, 8, 9] },
        };
        let a = Permutation::<O3>::identity();
        assert_eq!(result, a);
        println!("{}", a)
    }

    #[test]
    fn test_first_4() {
        let result: Permutation<O4> = Permutation {
            square: Square { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16] } ,
        };
        let a = Permutation::<O4>::identity();
        assert_eq!(result, a);
    }

    #[test]
    fn test_next_3() {
        let result: Permutation<O3> = Permutation {
            square: Square { data: [1, 2, 3, 4, 5, 6, 7, 9, 8] },
        };
        let mut a = Permutation::<O3>::identity();
        a = *a.next_perm().unwrap();
        assert_eq!(result, a);
    }

    #[test]
    fn test_next_4() {
        let result: Permutation<O4> = Permutation {
            square: Square { data: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 16, 15] } ,
        };
        let mut a = Permutation::<O4>::identity();
        a = *a.next_perm().unwrap();
        assert_eq!(result, a);
    }

    // #[test]
    // fn test_perm_iter_3() {
    //     let a = Permutation::<O3>::permutation_range(0, 8);
    //     let a_vec = a.collect::<Vec<Permutation<O3>>>();
    //     assert_eq!(8, a_vec.len());
    //     assert_eq!(7, a_vec.last().unwrap().index)
    // }

    // #[test]
    // fn test_perm_iter_4() {
    //     let a = Permutation::<O4>::permutation_range(0, 8);
    //     let a_vec = a.collect::<Vec<Permutation<O4>>>();
    //     assert_eq!(8, a_vec.len());
    //     assert_eq!(7, a_vec.last().unwrap().index)
    // }
}
