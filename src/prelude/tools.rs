use std::collections::BTreeSet;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{Permutation, O4};

#[must_use]
pub fn generate_permutation_isometries(perm: Permutation<O4>) -> BTreeSet<Permutation<O4>> {
    perm.generate_d().into_iter().collect()
}

#[must_use]
pub fn minimize_permutation_isometry(perm: Permutation<O4>) -> Permutation<O4> {
    *generate_permutation_isometries(perm).first().unwrap()
}

#[must_use]
pub fn reduce_isometry(origin: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    let mut unique_set = BTreeSet::new();
    for s in origin {
        if unique_set
            .intersection(&s.generate_d().into_iter().collect())
            .copied()
            .collect::<BTreeSet<_>>()
            .is_empty()
        {
            unique_set.insert(*s);
        }
    }
    unique_set
}

#[must_use]
pub fn compute_factors_ainvc(set: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    set.iter()
        .cartesian_product(set.iter())
        .par_bridge()
        .map(|(&a, &c)| a.inv() * c)
        .collect::<BTreeSet<_>>()
}

#[must_use]
pub fn extend_isometry(source: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    source
        .iter()
        .flat_map(|&item| item.generate_d().into_iter())
        .collect()
}
