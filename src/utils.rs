use std::collections::HashSet;
use std::hash::Hash;

use rand::{Rng, Rand};

#[allow(dead_code)]
pub fn random_unique_vec<R, T>(rng: &mut R, size: usize) -> Vec<T>
    where R: Rng,
          T: Rand + Hash + PartialEq + Eq,
{
    let mut values: HashSet<T> = HashSet::with_capacity(size);
    while values.len() < size {
        values.insert(rng.gen());
    }
    values.into_iter().collect()
}
