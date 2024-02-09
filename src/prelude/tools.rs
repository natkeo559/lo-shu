use std::collections::BTreeSet;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{Permutation, O4};

/// Filters a set to contain unique squares up to all possible rotations and reflections (isometries).
///
/// # Parameters:
/// origin: &BTreeSet<Permutation<O4>>
pub fn reduce_isometry(origin: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    let mut unique_set = BTreeSet::new();
    for s in origin.iter() {
        if unique_set
            .intersection(&s.generate_d().into_iter().collect())
            .map(|i| *i)
            .collect::<BTreeSet<_>>()
            .is_empty()
        {
            unique_set.insert(*s);
        }
    }
    unique_set
}

/// Computes the transformations between squares in a set.
///
/// Given a pair (a, c) from the set,
/// 
/// Factored action "b" from a * b = c
/// 
/// b = a.inv() * c
///
/// # Parameters
/// group: &BTreeSet<Permutation<O4>>
pub fn compute_group_actions(set: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    set
        .iter()
        .cartesian_product(set.iter())
        .par_bridge()
        .map(|(&a, &c)| a.inv() * c)
        .collect::<BTreeSet<_>>()
}

pub fn extend_isometry(source: &BTreeSet<Permutation<O4>>) {

}