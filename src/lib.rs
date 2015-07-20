#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate rand;
extern crate regex;

use std::collections::BTreeMap;
use std::ops::Add;
use quickcheck::Arbitrary;
use quickcheck::Gen;
use rand::Rng;
use std::str::FromStr;
use regex::Regex;
use std::num::ParseIntError;

pub trait Die {
    type Dist: Distribution;
    fn distribution(&self) -> Self::Dist;
    fn roll<R: Rng>(&self, rng: &mut R) -> i64;
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SimpleDie {
    pub size: i64,
    pub add: i64,
}

impl SimpleDie {
    pub fn new(size: i64, add: i64) -> SimpleDie {
        SimpleDie { size: size, add: add }
    }
}

impl Die for SimpleDie {
    type Dist = EqualDistribution;

    fn distribution(&self) -> Self::Dist {
        let min = 1 + self.add;
        let max = self.size + self.add;
        let members = (min..max+1).collect();
        EqualDistribution::new(members)
    }

    fn roll<R: Rng>(&self, rng: &mut R) -> i64 {
        rng.gen_range(1, self.size + 1) + self.add
    }
}

impl Add<i64> for SimpleDie {
    type Output = SimpleDie;
    fn add(self, add: i64) -> SimpleDie {
        SimpleDie { size: self.size, add: self.add + add }
    }
}

impl Add<SimpleDie> for i64 {
    type Output = SimpleDie;
    fn add(self, d: SimpleDie) -> SimpleDie {
        d + self
    }
}

impl Arbitrary for SimpleDie {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let size: i64 = Arbitrary::arbitrary(g);
        let size = size.abs() + 1;
        let add = Arbitrary::arbitrary(g);
        SimpleDie::new(size, add)
    }
}

impl FromStr for SimpleDie {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^d(?P<size>\d+)((?P<sign>[+-])(?P<add>\d+))?$").unwrap();
        let caps = re.captures(s).unwrap();

        let size: i64 = try!(caps.name("size").unwrap().parse());
        let mut add: i64 = try!(caps.name("add").unwrap_or("0").parse());
        if caps.name("sign").unwrap_or("+") == "-" {
            add = -add;
        }

        Result::Ok(SimpleDie::new(size, add))
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use quickcheck::TestResult;

    #[quickcheck]
    fn equal_distr_mean(d: SimpleDie) -> bool {
        let dist = d.distribution();
        dist.mean() == (d.size as f64) / 2.0 + (d.add as f64) + 0.5
    }

    #[quickcheck]
    fn add_dice_and_numbers(d1: SimpleDie, a: i64) -> bool {
        let d2 = d1 + a;
        let d3 = a + d1;
        d2.size == d1.size && d2.add == d1.add + a && d2 == d3
    }

    #[quickcheck]
    fn size_plus_add_is_max(d: SimpleDie) -> bool {
        let dist = d.distribution();
        d.size + d.add == dist.max()
    }

    #[quickcheck]
    fn one_plus_add_is_min(d: SimpleDie) -> bool {
        let dist = d.distribution();
        1 + d.add == dist.min()
    }

    #[quickcheck]
    fn rolling_dice_is_in_range(d: SimpleDie) -> bool {
        let mut rng = thread_rng();
        let dist = d.distribution();
        let roll = d.roll(&mut rng);
        dist.min() <= roll && roll <= dist.max()
    }

    #[quickcheck]
    fn parsing_bare_die(size: i64) -> TestResult {
        if size < 1 {
            return TestResult::discard();
        }
        let size = size.abs() + 1;
        let formatted = format!("d{}", size);
        let d: SimpleDie = formatted.parse().unwrap();
        TestResult::from_bool(d.size == size && d.add == 0)
    }

    #[quickcheck]
    fn parsing_bare_die_with_add(size: i64, add: i64) -> TestResult {
        if size < 1 || add < 1 {
            return TestResult::discard();
        }
        let formatted = format!("d{}+{}", size, add);
        let d: SimpleDie = formatted.parse().unwrap();
        TestResult::from_bool(d.size == size && d.add == add)
    }

    #[quickcheck]
    fn parsing_bare_die_with_subtract(size: i64, add: i64) -> TestResult {
        if size < 1 || add > -1 {
            return TestResult::discard();
        }
        let formatted = format!("d{}{}", size, add);
        let d: SimpleDie = formatted.parse().unwrap();
        TestResult::from_bool(d.size == size && d.add == add)
    }
}
