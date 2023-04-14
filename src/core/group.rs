use crate::{OrderFour, OrderThree, Permutation, Square, Transform};
use std::collections::HashSet;

pub trait Group
where
    Self: Sized,
{
    /// Generate the dihedral group for a Square or Permutation
    ///
    /// # Note:
    /// Because this function uses a HashSet, the result will contain non-trivial ordering
    fn generate_d(&self) -> HashSet<Self>;

    /// Generate the dihedral group for a Square or Permutation. The resulting HashSet will only contain the indexes of the associated Squares or Permutations.
    ///
    /// # Note:
    /// Because this function uses a HashSet, the result will contain non-trivial ordering
    fn generate_d_indexes(&self) -> HashSet<usize>;
}

macro_rules! impl_d {
    ($p:tt) => {
        impl Group for Square<$p> {
            fn generate_d(&self) -> HashSet<Square<$p>> {
                let mut set = HashSet::new();
                let mut a = *self;

                set.insert(a);
                set.insert(a.rotate_90());
                set.insert(a.rotate_90().rotate_90());
                set.insert(a.rotate_90().rotate_90().rotate_90());

                set.insert(a.reflect_x());
                set.insert(a.reflect_x().rotate_90());
                set.insert(a.reflect_x().rotate_90().rotate_90());
                set.insert(a.rotate_90().reflect_x());

                set
            }

            fn generate_d_indexes(&self) -> HashSet<usize> {
                let mut set = HashSet::new();
                let mut a = *self;

                set.insert(a.perm_id().index);
                set.insert(a.rotate_90().perm_id().index);
                set.insert(a.rotate_90().rotate_90().perm_id().index);
                set.insert(a.rotate_90().rotate_90().rotate_90().perm_id().index);

                set.insert(a.reflect_x().perm_id().index);
                set.insert(a.reflect_x().rotate_90().perm_id().index);
                set.insert(a.reflect_x().rotate_90().rotate_90().perm_id().index);
                set.insert(a.rotate_90().reflect_x().perm_id().index);

                set
            }
        }

        impl Group for Permutation<$p> {
            fn generate_d(&self) -> HashSet<Permutation<$p>> {
                let mut set = HashSet::new();
                let mut a = self.square;

                set.insert(a.perm_id());
                set.insert(a.rotate_90().perm_id());
                set.insert(a.rotate_90().rotate_90().perm_id());
                set.insert(a.rotate_90().rotate_90().rotate_90().perm_id());

                set.insert(a.reflect_x().perm_id());
                set.insert(a.reflect_x().rotate_90().perm_id());
                set.insert(a.reflect_x().rotate_90().rotate_90().perm_id());
                set.insert(a.rotate_90().reflect_x().perm_id());
                set
            }

            fn generate_d_indexes(&self) -> HashSet<usize> {
                let mut set = HashSet::new();
                let mut a = self.square;

                set.insert(a.perm_id().index);
                set.insert(a.rotate_90().perm_id().index);
                set.insert(a.rotate_90().rotate_90().perm_id().index);
                set.insert(a.rotate_90().rotate_90().rotate_90().perm_id().index);

                set.insert(a.reflect_x().perm_id().index);
                set.insert(a.reflect_x().rotate_90().perm_id().index);
                set.insert(a.reflect_x().rotate_90().rotate_90().perm_id().index);
                set.insert(a.rotate_90().reflect_x().perm_id().index);

                set
            }
        }
    };
}

impl_d!(OrderThree);
impl_d!(OrderFour);

#[cfg(test)]
mod test_group {
    use crate::{OrderThree, Permutation};

    use super::*;

    #[test]
    fn test_generate_d_3() {
        let a = Permutation::<OrderThree>::first();
        let b = a.square.generate_d();
        let mut c = HashSet::new();
        c.insert(a.square);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));

        let a = Permutation::<OrderThree>::first();
        let b = a.generate_d();
        let mut c = HashSet::new();
        c.insert(a);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }

    #[test]
    fn test_generate_d_indexes_3() {
        let a = Permutation::<OrderThree>::first();
        let b = a.generate_d_indexes();
        let mut c = HashSet::new();
        c.insert(a.index);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));

        let a = Permutation::<OrderThree>::first();
        let b = a.square.generate_d_indexes();
        let mut c = HashSet::new();
        c.insert(a.index);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }

    #[test]
    fn test_generate_d_4() {
        let a = Permutation::<OrderFour>::first();
        let b = a.square.generate_d();
        let mut c = HashSet::new();
        c.insert(a.square);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));

        let a = Permutation::<OrderFour>::first();
        let b = a.generate_d();
        let mut c = HashSet::new();
        c.insert(a);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }

    #[test]
    fn test_generate_d_indexes_4() {
        let a = Permutation::<OrderFour>::first();
        let b = a.generate_d_indexes();
        let mut c = HashSet::new();
        c.insert(a.index);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));

        let a = Permutation::<OrderFour>::first();
        let b = a.square.generate_d_indexes();
        let mut c = HashSet::new();
        c.insert(a.index);

        assert_eq!(8, b.len());
        assert!(!b.is_disjoint(&c));
    }
}
