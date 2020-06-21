pub fn collatz(n: u64) -> Option<u64> {
    let mut num = n;
    let mut steps = 0u64;
    loop {
        match num {
            0 => return None,
            1 => return Some(steps),
            n if n % 2 == 0 => num /= 2,
            _ => num = num * 3 + 1,
        }
        steps += 1;
    }
}
