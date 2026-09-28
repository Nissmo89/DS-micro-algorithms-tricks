pub struct FenwickTree {
    n: usize,
    bit: Vec<u64>,
}

impl FenwickTree {
    /// Creates a new Fenwick Tree (Binary Indexed Tree) of size `n`.
    /// All elements are initially zero.
    pub fn new(n: usize) -> Self {
        FenwickTree { n, bit: vec![0; n + 1] }
    }

    /// Adds `delta` to element at position `idx` (1‑based).
    pub fn add(&mut self, mut idx: usize, delta: u64) {
        while idx <= self.n {
            self.bit[idx] += delta;
            idx += idx & (!idx + 1); // idx += idx & -idx
        }
    }

    /// Returns the prefix sum [1, idx] (1‑based).
    pub fn sum(&self, mut idx: usize) -> u64 {
        let mut res = 0;
        while idx > 0 {
            res += self.bit[idx];
            idx -= idx & (!idx + 1); // idx -= idx & -idx
        }
        res
    }

    /// Returns the sum of the range [l, r] (1‑based, inclusive).
    pub fn range_sum(&self, l: usize, r: usize) -> u64 {
        if l > r { return 0; }
        self.sum(r) - self.sum(l - 1)
    }
}
