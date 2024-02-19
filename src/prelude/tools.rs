use std::collections::BTreeSet;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::{Enumerable, Permutation, O4};

/// Converts a k-set into a set of permutations.
///
/// Given a `BTreeSet<u64>` `set` and a `usize` `take`, returns a `BTreeSet` of permutations.
/// The function iterates over the elements of `set`, takes the first `take` elements, and maps
/// them to permutations using `Permutation::<O4>::kth`. Finally, it collects the resulting
/// permutations into a `BTreeSet` and returns it.
#[must_use]
pub fn k_set_to_permutation_set(set: &BTreeSet<u64>, take: usize) -> BTreeSet<Permutation<O4>> {
    set.iter()
        .take(take)
        .map(|&a| Permutation::<O4>::kth(a))
        .collect::<BTreeSet<_>>()
}

/// Generates the isometries of a permutation.
///
/// Given a reference to a `Permutation<O4>` `perm`, generates the isometries of `perm` and
/// collects them into a `BTreeSet` before returning it.
#[must_use]
pub fn generate_permutation_isometries(perm: &Permutation<O4>) -> BTreeSet<Permutation<O4>> {
    perm.generate_d().into_iter().collect()
}

/// Minimizes the isometry of a permutation.
///
/// Given a reference to a `Permutation<O4>` `perm`, generates its isometries, selects the first
/// isometry (the one that minimizes k in the ordered set), and returns it. If no isometries are
/// found, for whatever reason, returns the identity permutation.
#[must_use]
pub fn minimize_permutation_isometry(perm: &Permutation<O4>) -> Permutation<O4> {
    *generate_permutation_isometries(perm)
        .first()
        .unwrap_or(&Permutation::identity())
}

/// Minimizes the isometries of a set of permutations.
///
/// Given a reference to a `BTreeSet<Permutation<O4>>` `set`, iterates over its elements, minimizes
/// each permutation isometry, and collects the results into a new `BTreeSet` before returning it.
#[must_use]
pub fn minimize_set_isometry(set: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    set.iter()
        .map(|&p| minimize_permutation_isometry(&p))
        .collect()
}

/// Reduces a set of isometries to a minimal set.
///
/// Given a reference to a `BTreeSet<Permutation<O4>>` `origin`, iterates over its elements,
/// removing any isometry that is a combination of other isometries already in the set. Returns a
/// new `BTreeSet` containing the reduced set of isometries.
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

/// Computes the transformations between squares in a set.
///
/// Given a reference to a `BTreeSet<Permutation<O4>>` `set`, computes the transformation `b` from
/// the factored action `a * b = c` for each pair of permutations `a` and `c` in the set. Returns a
///  ew `BTreeSet` containing the computed transformations.
#[must_use]
pub fn compute_factors_ainvc(set: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    set.iter()
        .cartesian_product(set.iter())
        .par_bridge()
        .map(|(&a, &c)| a.inv() * c)
        .collect::<BTreeSet<_>>()
}

/// Extends a set of permutations with their associated isometries.
///
/// Given a reference to a `BTreeSet<Permutation<O4>>` `source`, iterates over its elements,
/// generates the isometries for each element, and collects them into a new `BTreeSet` before
/// returning it.
#[must_use]
pub fn extend_isometry(source: &BTreeSet<Permutation<O4>>) -> BTreeSet<Permutation<O4>> {
    source
        .iter()
        .flat_map(|&item| item.generate_d().into_iter())
        .collect()
}
