use std::collections::HashSet;

use quickcheck::{Arbitrary, Gen};

use distributions::{Distribution, EqualDistribution};

impl Arbitrary for EqualDistribution {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let size: usize = Arbitrary::arbitrary(g);
        let size = size + 1;
        let mut values = HashSet::with_capacity(size);
        while values.len() < size {
            values.insert(Arbitrary::arbitrary(g));
        }
        EqualDistribution::new(values.iter().map(|x| { *x }).collect())
    }
}

#[quickcheck]
fn equal_distr_numerators(d: EqualDistribution) -> bool {
    d.numerators().values().all(|p| { *p == 1 })
}

#[quickcheck]
fn equal_distr_denominator(d: EqualDistribution) -> bool {
    d.denominator() == d.members.len() as i64
}
