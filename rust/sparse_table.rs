use std::cmp::min;

pub struct SparseTable {
    n: usize,
    log: Vec<usize>,
    st: Vec<Vec<i32>>,
}

impl SparseTable {
    pub fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let mut log = vec![0; n + 1];
        for i in 2..=n {
            log[i] = log[i / 2] + 1;
        }
        let k = log[n] + 1;
        let mut st = vec![vec![0; n]; k];
        for i in 0..n {
            st[0][i] = arr[i];
        }
        for j in 1..k {
            let len = 1 << j;
            for i in 0..=n - len {
                st[j][i] = min(st[j - 1][i], st[j - 1][i + (len >> 1)]);
            }
        }
        Self { n, log, st }
    }

    pub fn query(&self, l: usize, r: usize) -> i32 {
        let len = r - l + 1;
        let k = self.log[len];
        min(self.st[k][l], self.st[k][r - (1 << k) + 1])
    }
}
