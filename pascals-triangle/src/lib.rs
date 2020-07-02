pub struct PascalsTriangle {
    triangle: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            triangle: (0..row_count)
                .map(|n| (0..=n).map(|k| c(n, k)).collect())
                .collect(),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.triangle.clone()
    }
}

pub fn c(n: u32, k: u32) -> u32 {
    factorial(n) / (factorial(k) * factorial(n - k))
}

pub fn factorial(n: u32) -> u32 {
    (2..=n).fold(1, |p, q| p * q)
}
