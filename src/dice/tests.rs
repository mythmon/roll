use quickcheck::{Arbitrary, Gen, TestResult};
use rand::thread_rng;

use distributions::Distribution;
use dice::{Die, SimpleDie};

impl Arbitrary for SimpleDie {
    fn arbitrary<G: Gen>(g: &mut G) -> Self {
        let size: i64 = Arbitrary::arbitrary(g);
        let size = size.abs() + 1;
        let add = Arbitrary::arbitrary(g);
        SimpleDie::new(size, add)
    }
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
