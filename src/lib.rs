#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate rand;
extern crate regex;

mod dice;
mod distributions;
mod utils;

pub use dice::{Die, SimpleDie};
pub use distributions::{Distribution, EqualDistribution};
