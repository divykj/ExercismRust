pub fn nth(n: u32) -> u32 {
    let mut i = 3;
    let mut j = 0;
    let mut jth_prime = 2;
    while j < n {
        if is_prime(i) {
            j += 1;
            jth_prime = i;
        }
        i += 1;
    }
    jth_prime
}

pub fn is_prime(n: u32) -> bool {
    for i in 2..(n - 1) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
