#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate rand;
extern crate regex;

use std::ops::Add;
use quickcheck::Gen;
use rand::Rng;
use std::str::FromStr;
use regex::Regex;
use std::num::ParseIntError;
use super::{Distribution, EqualDistribution};

#[cfg(test)] mod tests;

pub trait Die {
    type Dist: Distribution;
    fn distribution(&self) -> Self::Dist;
    fn roll<R: Rng>(&self, rng: &mut R) -> i64;
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
