use std::collections::HashSet;

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut primes = Vec::<u64>::new();
    let mut discarded = HashSet::<u64>::new();

    for i in 2..=upper_bound {
        if !discarded.contains(&i) {
            primes.push(i);
            discarded.extend((i..=upper_bound).step_by(i as usize));
        }
    }

    primes
}
