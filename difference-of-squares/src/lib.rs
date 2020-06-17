pub fn square_of_sum(n: u32) -> u32 {
    // u32::pow((1..=n).sum(), 2) // Using loop
    (n * (n + 1) / 2).pow(2) // Using formula
}

pub fn sum_of_squares(n: u32) -> u32 {
    // (1..=n).map(|n| n * n).sum() // Using loop
    n * (n + 1) * (2 * n + 1) / 6 // Using formula
}

pub fn difference(n: u32) -> u32 {
    // square_of_sum(n) - sum_of_squares(n) // Using functions
    (n.pow(4) - n.pow(2)) / 4 + (n.pow(3) - n) / 6 // Using formula
}
