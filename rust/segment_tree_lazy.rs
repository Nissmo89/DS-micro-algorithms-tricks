use std::cmp::max;

pub struct SegmentTree {
    n: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    pub fn new(arr: &[i64]) -> Self {
        let n = arr.len();
        let size = 4 * n;
        let mut st = SegmentTree {
            n,
            tree: vec![0; size],
            lazy: vec![0; size],
        };
        st.build(1, 0, n - 1, arr);
        st
    }

    fn build(&mut self, node: usize, l: usize, r: usize, arr: &[i64]) {
        if l == r {
            self.tree[node] = arr[l];
        } else {
            let mid = (l + r) / 2;
            self.build(node * 2, l, mid, arr);
            self.build(node * 2 + 1, mid + 1, r, arr);
            self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
        }
    }

    pub fn update_range(&mut self, ql: usize, qr: usize, val: i64) {
        self.update(1, 0, self.n - 1, ql, qr, val);
    }

    fn update(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize, val: i64) {
        if self.lazy[node] != 0 {
            self.tree[node] += (r - l + 1) as i64 * self.lazy[node];
            if l != r {
                self.lazy[node * 2] += self.lazy[node];
                self.lazy[node * 2 + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
        if r < ql || l > qr {
            return;
        }
        if ql <= l && r <= qr {
            self.tree[node] += (r - l + 1) as i64 * val;
            if l != r {
                self.lazy[node * 2] += val;
                self.lazy[node * 2 + 1] += val;
            }
            return;
        }
        let mid = (l + r) / 2;
        self.update(node * 2, l, mid, ql, qr, val);
        self.update(node * 2 + 1, mid + 1, r, ql, qr, val);
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1];
    }

    pub fn query_range(&mut self, ql: usize, qr: usize) -> i64 {
        self.query(1, 0, self.n - 1, ql, qr)
    }

    fn query(&mut self, node: usize, l: usize, r: usize, ql: usize, qr: usize) -> i64 {
        if self.lazy[node] != 0 {
            self.tree[node] += (r - l + 1) as i64 * self.lazy[node];
            if l != r {
                self.lazy[node * 2] += self.lazy[node];
                self.lazy[node * 2 + 1] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }
        if r < ql || l > qr {
            return 0;
        }
        if ql <= l && r <= qr {
            return self.tree[node];
        }
        let mid = (l + r) / 2;
        let left = self.query(node * 2, l, mid, ql, qr);
        let right = self.query(node * 2 + 1, mid + 1, r, ql, qr);
        left + right
    }
}
