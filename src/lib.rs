#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate rand;
extern crate regex;

mod dice;
mod distributions;

pub use dice::{Die, SimpleDie};
pub use distributions::{Distribution, EqualDistribution};
