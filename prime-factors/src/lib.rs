pub fn factors(n: u64) -> Vec<u64> {
    let mut facts: Vec<u64> = Vec::new();
    let mut rem: u64 = n;
    let mut i: u64 = 2;
    while rem != 1 {
        if rem % i == 0 {
            facts.push(i);
            rem /= i;
        } else {
            i += 1;
        }
    }
    facts
}
