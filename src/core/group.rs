use crate::{OrderThree, Permutation, Square, Transform};
use std::collections::HashSet;

pub trait Group
where
    Self: Sized,
{
    fn generate_d(&self) -> HashSet<Self>;
}

impl Group for Square<u8, OrderThree> {
    fn generate_d(&self) -> HashSet<Square<u8, OrderThree>> {
        let mut set = HashSet::new();
        let mut a = *self;

        set.insert(a);
        set.insert(a.rotate_90());
        set.insert(a.rotate_90().rotate_90());
        set.insert(a.rotate_90().rotate_90().rotate_90());

        set.insert(a.reflect_x());
        set.insert(a.reflect_x().rotate_90());
        set.insert(a.reflect_x().rotate_90().rotate_90());
        set.insert(a.reflect_x().rotate_90().rotate_90().rotate_90());

        set
    }
}

impl Group for Permutation<u8, OrderThree> {
    fn generate_d(&self) -> HashSet<Permutation<u8, OrderThree>> {
        let mut set = HashSet::new();
        let mut a = self.square;

        set.insert(a.perm_id());
        set.insert(a.rotate_90().perm_id());
        set.insert(a.rotate_90().rotate_90().perm_id());
        set.insert(a.rotate_90().rotate_90().rotate_90().perm_id());

        set.insert(a.reflect_x().perm_id());
        set.insert(a.reflect_x().rotate_90().perm_id());
        set.insert(a.reflect_x().rotate_90().rotate_90().perm_id());
        set.insert(a.reflect_x().rotate_90().rotate_90().rotate_90().perm_id());
        set
    }
}

#[cfg(test)]
mod test_group {
    use crate::{OrderThree, Permutation};

    use super::*;

    #[test]
    fn test_d() {
        let a = Permutation::<u8, OrderThree>::first();
        let b = a.square.generate_d();
        assert_eq!(8, b.len());

        let mut c = HashSet::new();
        c.insert(a.square);
        assert!(!b.is_disjoint(&c));

        let d = Permutation::<u8, OrderThree>::kth(4);
        c.clear();
        c.insert(d.square);

        assert!(b.is_disjoint(&c))
    }
}
