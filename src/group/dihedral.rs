use crate::{Params, Permutation};
use std::collections::{BTreeSet, HashSet};

impl<P: Params + Copy> Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    /// Generate the dihedral group for a Square or Permutation
    ///
    /// # Note:
    /// Because this function uses a HashSet, the result will contain non-trivial ordering
    pub fn generate_d(&self) -> HashSet<Permutation<P>> {
        let mut set = HashSet::new();
        let a = *self;
        let r = Self::identity().square.rotate_90().to_perm();
        let s = Self::identity().square.reflect_x().to_perm();

        set.insert(a);
        set.insert(a * r);
        set.insert(a * r.pow(2));
        set.insert(a * r.pow(3));

        set.insert(a * s);
        set.insert(a * s * r);
        set.insert(a * s * r.pow(2));
        set.insert(a * r * s);
        set
    }

    /// Generate the dihedral group for a Square or Permutation. The resulting HashSet will only contain the indexes of the associated Permutations.
    ///
    /// # Note:
    /// Because this function uses a HashSet, the result will contain non-trivial ordering
    pub fn generate_d_indexes(&self) -> BTreeSet<usize> {
        let mut set = BTreeSet::new();
        let a = *self;
        let r = Self::identity().square.rotate_90().to_perm();
        let s = Self::identity().square.reflect_x().to_perm();

        set.insert(a.index);
        set.insert((a * r).index);
        set.insert((a * r.pow(2)).index);
        set.insert((a * r.pow(3)).index);

        set.insert((a * s).index);
        set.insert((a * s * r).index);
        set.insert((a * s * r.pow(2)).index);
        set.insert((a * r * s).index);
        set
    }
}

#[cfg(test)]
mod test_group {
    use crate::{OrderFour, OrderThree, Permutation};

    use super::*;

    #[test]
    fn test_generate_d_3() {
        let a = Permutation::<OrderThree>::identity();
        let b = a.generate_d();
        let mut c = HashSet::new();
        c.insert(a);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }

    #[test]
    fn test_generate_d_indexes_3() {
        let a = Permutation::<OrderThree>::identity();
        let b = a.generate_d_indexes();
        let mut c = BTreeSet::new();
        c.insert(a.index);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }

    #[test]
    fn test_generate_d_4() {
        let a = Permutation::<OrderFour>::identity();
        let b = a.generate_d();
        let mut c = HashSet::new();
        c.insert(a);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }

    #[test]
    fn test_generate_d_indexes_4() {
        let a = Permutation::<OrderFour>::identity();
        let b = a.generate_d_indexes();
        let mut c = BTreeSet::new();
        c.insert(a.index);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }
}
