use crate::{Params, Permutation, Transform};
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

// impl<P: Params + Copy> Group for Square<P> where [(); P::ELEMENTS]:{
//     fn generate_d(&self) -> HashSet<Square<P>> {
//         let mut set = HashSet::new();
//         let mut a = *self;
//         let r = Permutation::<P>::
//         let s

//         set.insert(a);
//         set.insert(a * r);
//         set.insert(a.rotate_90().rotate_90());
//         set.insert(a.rotate_90().rotate_90().rotate_90());

//         set.insert(a.reflect_x());
//         set.insert(a.reflect_x().rotate_90());
//         set.insert(a.reflect_x().rotate_90().rotate_90());
//         set.insert(a.rotate_90().reflect_x());

//         set
//     }

//     fn generate_d_indexes(&self) -> HashSet<usize> {
//         let mut set = HashSet::new();
//         let mut a = *self;

//         set.insert(a.to_perm().index);
//         set.insert(a.rotate_90().to_perm().index);
//         set.insert(a.rotate_90().rotate_90().to_perm().index);
//         set.insert(a.rotate_90().rotate_90().rotate_90().to_perm().index);

//         set.insert(a.reflect_x().to_perm().index);
//         set.insert(a.reflect_x().rotate_90().to_perm().index);
//         set.insert(a.reflect_x().rotate_90().rotate_90().to_perm().index);
//         set.insert(a.rotate_90().reflect_x().to_perm().index);

//         set
//     }
// }

impl<P: Params + Copy> Group for Permutation<P>
where
    [(); P::ELEMENTS]:,
{
    fn generate_d(&self) -> HashSet<Permutation<P>> {
        let mut set = HashSet::new();
        let a = *self;
        let r = Self::identity().square.rotate_90().to_perm();
        let s = Self::identity().square.reflect_x().to_perm();

        set.insert(a);
        set.insert(a * r);
        set.insert(a * r * r);
        set.insert(a * r * r * r);

        set.insert(a * s);
        set.insert(a * s * r);
        set.insert(a * s * r * r);
        set.insert(a * r * s);
        set
    }

    fn generate_d_indexes(&self) -> HashSet<usize> {
        let mut set = HashSet::new();
        let a = *self;
        let r = Self::identity().square.rotate_90().to_perm();
        let s = Self::identity().square.reflect_x().to_perm();

        set.insert(a.index);
        set.insert((a * r).index);
        set.insert((a * r * r).index);
        set.insert((a * r * r * r).index);

        set.insert((a * s).index);
        set.insert((a * s * r).index);
        set.insert((a * s * r * r).index);
        set.insert((a * r * s).index);
        set
    }
}

// #[cfg(test)]
// mod test_group {
//     use crate::{OrderThree, OrderFour, Permutation};

//     use super::*;

// #[test]
// fn test_generate_d_3() {
//     let a = Permutation::<OrderThree>::identity();
//     let b = a.square.generate_d();
//     let mut c = HashSet::new();
//     c.insert(a.square);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));

//     let a = Permutation::<OrderThree>::identity();
//     let b = a.generate_d();
//     let mut c = HashSet::new();
//     c.insert(a);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));
// }

// #[test]
// fn test_generate_d_indexes_3() {
//     let a = Permutation::<OrderThree>::identity();
//     let b = a.generate_d_indexes();
//     let mut c = HashSet::new();
//     c.insert(a.index);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));

//     let a = Permutation::<OrderThree>::identity();
//     let b = a.square.generate_d_indexes();
//     let mut c = HashSet::new();
//     c.insert(a.index);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));
// }

// #[test]
// fn test_generate_d_4() {
//     let a = Permutation::<OrderFour>::identity();
//     let b = a.square.generate_d();
//     let mut c = HashSet::new();
//     c.insert(a.square);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));

//     let a = Permutation::<OrderFour>::identity();
//     let b = a.generate_d();
//     let mut c = HashSet::new();
//     c.insert(a);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));
// }

// #[test]
// fn test_generate_d_indexes_4() {
//     let a = Permutation::<OrderFour>::identity();
//     let b = a.generate_d_indexes();
//     let mut c = HashSet::new();
//     c.insert(a.index);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));

//     let a = Permutation::<OrderFour>::identity();
//     let b = a.square.generate_d_indexes();
//     let mut c = HashSet::new();
//     c.insert(a.index);

//     assert_eq!(8, b.len());
//     assert!(!b.is_disjoint(&c));
// }
// }
