#![plugin(quickcheck_macros)]

extern crate quickcheck;
extern crate rand;
extern crate regex;

use rand::Rng;

use distributions::Distribution;

#[cfg(test)] mod tests;
mod simple;

pub use self::simple::SimpleDie;

pub trait Die {
    type Dist: Distribution;
    fn distribution(&self) -> Self::Dist;
    fn roll<R: Rng>(&self, rng: &mut R) -> i64;
}
