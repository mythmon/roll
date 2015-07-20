extern crate roll;
extern crate rand;

use std::io::stdin;
use rand::thread_rng;
use roll::{SimpleDie, Die, Distribution};

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut rng = thread_rng();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap_or_else(|e| { panic!(e); });

    let die: SimpleDie = input.trim().parse().unwrap_or_else(|e| { panic!(e); });
    let distr = die.distribution();

    println!("Mean: {}", distr.mean());
    println!("Range: {} to {}", distr.min(), distr.max());

    print!("Sample rolls: ");
    for _ in 0..9  {
        print!("{}, ", die.roll(&mut rng));
    }
    println!("{}", die.roll(&mut rng));
}
