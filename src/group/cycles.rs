use crate::{Params, Permutation, Square};
use std::mem::swap;
use std::{collections::HashMap, fmt, marker::PhantomData};

/// A generic struct representing formal permutation presentation based on parameters `P`
/// implementing the `Params` trait.
///
/// # Examples
///
/// ```
/// use lo_shu::{Cycles, Square, O4};
///
/// let a = Cycles::from_vecs(vec![
///     vec![1, 4],
///     vec![2, 14],
///     vec![3, 15],
///     vec![5, 9],
///     vec![6, 7],
///     vec![8, 12],
///     vec![10, 11],
///     vec![13, 16],
/// ]);
///
/// let b = Square::<O4>::from_array([4, 14, 15, 1, 9, 7, 6, 12, 5, 11, 10, 8, 16, 2, 3, 13])
///     .to_perm()
///     .cyclic_notation();
///
/// assert_eq!(a, b);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycles<P: Params> {
    /// The vector of cycles represented as vectors of usize.
    pub k: Vec<Vec<usize>>,
    phantom: PhantomData<P>,
}

impl<P: Params + Copy> Cycles<P> {
    /// Calculates the greatest common divisor of two integers.
    #[inline]
    fn gcd(mut a: usize, mut b: usize) -> usize {
        if a == b {
            return a;
        }
        if b > a {
            swap(&mut a, &mut b);
        }
        while b > 0 {
            let tmp = a;
            a = b;
            b = tmp % b;
        }
        a
    }

    /// Calculates the least common multiple of two integers.
    #[inline]
    fn lcm(a: usize, b: usize) -> usize {
        a * (b / Self::gcd(a, b))
    }

    /// Computes the order of the cycles.
    #[must_use]
    pub fn order(&self) -> usize {
        let lens = self.cycle_lengths();
        lens.into_iter().fold(1, |a, b| Self::lcm(a, b))
    }

    /// Computes the weight of the cycles.
    #[must_use]
    pub fn weight(&self) -> usize {
        self.k.len()
    }

    /// Creates a new `Cycles` instance from a vector of vectors of usize.
    #[must_use]
    pub fn from_vecs(vecs: Vec<Vec<usize>>) -> Self {
        Self {
            k: vecs,
            phantom: PhantomData,
        }
    }

    /// Adds a new cycle to the `Cycles` instance.
    pub fn push(&mut self, value: Vec<usize>) {
        self.k.push(value);
    }

    /// Converts the `Cycles` instance into a `Permutation`.
    #[must_use]
    pub fn into_permutation(&mut self) -> Permutation<P>
    where
        [(); P::ELEMENTS]:,
    {
        let mut s = [0; P::ELEMENTS];
        // self.k.reverse();
        for (index, item) in s.iter_mut().enumerate() {
            *item = (index + 1) as u32;
            for j in &self.k {
                if let Some(p) = j.iter().position(|&a| a == (index + 1)) {
                    let next = (p + 1) % j.len();
                    *item = j[next] as u32;
                }
            }
        }

        Square::<P>::from_array(s).to_perm()
    }

    /// Retrieves the lengths of the cycles.
    #[must_use]
    pub fn cycle_lengths(&self) -> Vec<usize> {
        self.k.iter().map(std::vec::Vec::len).collect()
    }
}

impl<P: Params> fmt::Display for Cycles<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let d = &self.k;

        let mut output = String::new();
        for i in d {
            let mut s = String::new();
            s.push('(');
            for (index, elem) in i.iter().enumerate() {
                s.push_str(&elem.to_string());
                if index + 1 != i.len() {
                    s.push_str(", ");
                }
            }
            s.push(')');

            output.push_str(&s);
        }

        write!(f, "{output}")
    }
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    /// Converts the Permutation into cyclic notation represented by Cycles.
    /// # Panics
    /// - If key to corresponding map does not exist.
    #[must_use]
    pub fn cyclic_notation(&self) -> Cycles<P> {
        let hb: HashMap<usize, usize> = (1..=P::ELEMENTS)
            .zip((self.square.data).into_iter().map(|a| a as usize))
            .collect();
        let b = (1..=P::ELEMENTS).zip((self.square.data).into_iter().map(|a| a as usize));

        let mut all = vec![];
        let mut cycle = vec![];
        let mut taken = vec![];

        for pair in b {
            if !taken.contains(&pair) {
                let mut k = pair.0;
                for _ in 0..P::ELEMENTS {
                    let v = *hb.get(&k).unwrap();
                    if cycle.contains(&v) {
                        break;
                    }
                    cycle.push(v);
                    taken.push((k, v));
                    k = v;
                }
                let s = cycle
                    .clone()
                    .iter()
                    .position(|&a| &a == cycle.clone().iter().min().unwrap())
                    .unwrap();
                cycle.rotate_left(s);
                if cycle.len() > 1 {
                    all.push(cycle.clone());
                }
                cycle.clear();
            }
        }

        Cycles {
            k: all,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod test_ops {
    use crate::{CheckVector, Cycles, Enumerable, Permutation, Square, O3, O4};
    use rayon::prelude::*;

    #[test]
    #[ignore = "debugging"]
    fn test_display() {
        let a = Cycles::<O3>::from_vecs(vec![
            vec![1, 4],
            vec![2, 14],
            vec![3, 15],
            vec![5, 9],
            vec![6, 7],
            vec![8, 12],
            vec![10, 11],
            vec![13, 16],
        ]);

        println!("{}", a)
    }

    #[test]
    fn test_cyclic() {
        let a_s = Cycles::from_vecs(vec![vec![1, 4, 5, 3, 2, 9, 6, 7, 8]]);
        let a = Square::<O3>::from_array([4, 9, 2, 5, 3, 7, 8, 1, 6])
            .to_perm()
            .cyclic_notation();
        assert_eq!(a_s, a);

        let b_s = Cycles::from_vecs(vec![vec![1, 4, 3, 2, 9, 6, 7, 8]]);
        let b = Square::<O3>::from_array([4, 9, 2, 3, 5, 7, 8, 1, 6])
            .to_perm()
            .cyclic_notation();
        assert_eq!(b_s, b);

        let c_s = Cycles::from_vecs(vec![
            vec![1, 4],
            vec![2, 14],
            vec![3, 15],
            vec![5, 9],
            vec![6, 7],
            vec![8, 12],
            vec![10, 11],
            vec![13, 16],
        ]);
        let c = Square::<O4>::from_array([4, 14, 15, 1, 9, 7, 6, 12, 5, 11, 10, 8, 16, 2, 3, 13])
            .to_perm()
            .cyclic_notation();
        assert_eq!(c_s, c);
    }

    #[test]
    fn test_valid_cycles() {
        let b = (0..362880)
            .into_par_iter()
            .filter_map(|k| Permutation::<O3>::kth(k).check_v())
            .collect::<Vec<_>>();

        for i in b {
            println!("{:?}", i.cyclic_notation().k)
        }
    }

    #[test]
    fn test_into_perm() {
        let a = Permutation::<O3>::kth(50000);
        let b = a.cyclic_notation();
        let c = b.clone().into_permutation();

        assert_eq!(a, c)
    }

    #[test]
    fn test_order() {
        let a = Permutation::<O3>::kth(310011);
        assert_eq!(a.cyclic_notation().order(), 15)
    }
}
