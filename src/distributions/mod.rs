use std::collections::BTreeMap;

#[cfg(test)] mod tests;

pub trait Distribution {
    fn numerators(&self) -> BTreeMap<i64, i64>;
    fn denominator(&self) -> i64;

    fn min(&self) -> i64 {
        self.numerators().iter().map(|(n, _)| { *n }).min().unwrap_or(0)
    }

    fn max(&self) -> i64 {
        self.numerators().iter().map(|(n, _)| { *n }).max().unwrap_or(0)
    }

    fn mean(&self) -> f64 {
        let mut sum = 0;
        let mut div = 0;
        for (n, p) in self.numerators() {
            sum += n;
            div += p;
        }
        (sum as f64) / (div as f64)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EqualDistribution {
    members: Vec<i64>,
}

impl EqualDistribution {
    pub fn new(members: Vec<i64>) -> EqualDistribution {
        EqualDistribution { members: members }
    }
}

impl Distribution for EqualDistribution {
    fn numerators(&self) -> BTreeMap<i64, i64> {
        let mut map = BTreeMap::new();
        for n in self.members.iter() {
            map.insert(*n, 1);
        }
        map
    }

    fn denominator(&self) -> i64 {
        self.members.len() as i64
    }
}
