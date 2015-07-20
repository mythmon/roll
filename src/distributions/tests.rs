use quickcheck::{Arbitrary, Gen};

use distributions::{Distribution, EqualDistribution};
use utils;

impl Arbitrary for EqualDistribution {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let size: usize = Arbitrary::arbitrary(g);
        let size = size + 1;
        let values = utils::random_unique_vec(g, size);
        EqualDistribution::new(values)
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
