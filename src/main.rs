extern crate dice;
extern crate rand;

use std::io::stdin;
use rand::thread_rng;
use dice::{SimpleDie,Die};

#[cfg_attr(test, allow(dead_code))]
fn main() {
    let mut rng = thread_rng();
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap_or_else(|e| { panic!(e); });

    let die: SimpleDie = input.trim().parse().unwrap_or_else(|e| { panic!(e); });
    for _ in 0..10  {
        println!("{}", die.roll(&mut rng));
    }
}
