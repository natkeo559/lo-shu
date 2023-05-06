use crate::{Params, Permutation, Square, Transform};
use std::{collections::HashMap, fmt, marker::PhantomData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cycles<P: Params> {
    pub k: Vec<Vec<usize>>,
    phantom: PhantomData<P>,
}

impl<P: Params + Copy> Cycles<P> {
    pub fn new() -> Self {
        Self {
            k: vec![],
            phantom: PhantomData,
        }
    }

    pub fn from_vecs(vecs: Vec<Vec<usize>>) -> Self {
        Self {
            k: vecs,
            phantom: PhantomData,
        }
    }

    pub fn push(&mut self, value: Vec<usize>) {
        self.k.push(value)
    }

    pub fn into_permutation(&mut self) -> Permutation<P>
    where
        [(); P::ELEMENTS]:,
    {
        let mut s = [0; P::ELEMENTS];
        // self.k.reverse();
        for (index, item) in s.iter_mut().enumerate() {
            *item = (index + 1) as u8;
            for j in &self.k {
                if let Some(p) = j.iter().position(|&a| a == (index + 1)) {
                    let next = (p + 1) % j.len();
                    *item = j[next] as u8;
                }
            }
        }

        Square::<P>::from_array(s).to_perm()
    }
}

impl<P: Params> fmt::Display for Cycles<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.k)
    }
}

impl<P: Params + Copy> Default for Cycles<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    pub fn cyclic_notation(&self) -> Cycles<P> {
        let hb: HashMap<usize, usize> = (1..=P::ELEMENTS)
            .zip((self.square.0).into_iter().map(|a| a as usize))
            .collect();
        let b = (1..=P::ELEMENTS).zip((self.square.0).into_iter().map(|a| a as usize));

        let mut all = Cycles::<P>::new();
        let mut cycle = vec![];
        let mut taken = vec![];

        for pair in b {
            if !taken.contains(&pair) {
                let mut k = pair.0;
                for _ in 0..P::ELEMENTS {
                    let v = *hb.get(&k).unwrap();
                    if !cycle.contains(&v) {
                        cycle.push(v);
                        taken.push((k, v));
                        k = v;
                    } else {
                        break;
                    }
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

        all
    }
}

#[cfg(test)]
mod test_ops {
    use crate::{Check, Cycles, OrderFour, OrderThree, Params, Permutation, Square, Transform};
    use rayon::prelude::*;

    #[test]
    fn test_cyclic() {
        let a_s = Cycles::from_vecs(vec![vec![1, 4, 5, 3, 2, 9, 6, 7, 8]]);
        let a = Square::<OrderThree>::from_array([4, 9, 2, 5, 3, 7, 8, 1, 6])
            .to_perm()
            .cyclic_notation();
        assert_eq!(a_s, a);

        let b_s = Cycles::from_vecs(vec![vec![1, 4, 3, 2, 9, 6, 7, 8]]);
        let b = Square::<OrderThree>::from_array([4, 9, 2, 3, 5, 7, 8, 1, 6])
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
        let c = Square::<OrderFour>::from_array([
            4, 14, 15, 1, 9, 7, 6, 12, 5, 11, 10, 8, 16, 2, 3, 13,
        ])
        .to_perm()
        .cyclic_notation();
        assert_eq!(c_s, c);
    }

    #[test]
    fn cycle_tests() {
        let b = (0..OrderThree::PERMUTATIONS)
            .into_par_iter()
            .filter_map(|k| Permutation::<OrderThree>::kth(k).check())
            .collect::<Vec<_>>();

        for i in b {
            println!("{:?}", i.cyclic_notation().k)
        }
    }

    #[test]
    fn test_into_perm() {
        let a = Permutation::<OrderThree>::kth(50000);
        println!("{:?}", a);

        let b = a.cyclic_notation();
        println!("{}", b);

        let c = b.clone().into_permutation();
        println!("{:?}", c);

        assert_eq!(a, c)
    }
}
