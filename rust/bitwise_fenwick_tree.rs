use std::io::{self, Read};

struct FenwickTree {
    n: usize,
    bit: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        FenwickTree { n, bit: vec![0; n + 1] }
    }

    // add `delta` to element at index `idx` (0‑based)
    fn update(&mut self, idx: usize, delta: i64) {
        let mut i = idx + 1;
        while i <= self.n {
            self.bit[i] += delta;
            i += i & (!i + 1); // i += i & -i
        }
    }

    // sum of elements in [0, idx] inclusive (0‑based)
    fn query(&self, idx: usize) -> i64 {
        let mut res = 0;
        let mut i = idx + 1;
        while i > 0 {
            res += self.bit[i];
            i &= i - 1; // i -= i & -i
        }
        res
    }

    // sum of elements in [l, r] inclusive
    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.query(r) - if l == 0 { 0 } else { self.query(l - 1) }
    }
}

fn main() {
    // Example usage: read n and q, then process queries
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut ft = FenwickTree::new(n);
    for _ in 0..q {
        let t: i32 = iter.next().unwrap().parse().unwrap();
        if t == 1 {
            let idx: usize = iter.next().unwrap().parse().unwrap();
            let val: i64 = iter.next().unwrap().parse().unwrap();
            ft.update(idx, val);
        } else {
            let l: usize = iter.next().unwrap().parse().unwrap();
            let r: usize = iter.next().unwrap().parse().unwrap();
            println!("{}", ft.range_query(l, r));
        }
    }
}
