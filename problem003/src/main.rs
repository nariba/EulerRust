extern crate primes;

use std::cmp;
use primes::PrimeSet;

fn main() {
    let num = 600851475143;

    let mut pset = PrimeSet::new();
    let mut n = pset.prime_factors(num);
    let mut max = n[0];
    let mut n_iter = n.iter();

    while let Some(i) =  n_iter.next() {
        max = cmp::max(*i, max);
    }
    println!("{}", max);
}
