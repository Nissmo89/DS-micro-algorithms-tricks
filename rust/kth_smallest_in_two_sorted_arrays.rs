use std::cmp::Ordering;

/// Returns the k-th smallest element (1-indexed) from two sorted slices.
/// Assumes 1 <= k <= a.len() + b.len().
fn kth_smallest<T: Ord + Copy>(a: &[T], b: &[T], k: usize) -> T {
    let mut ia = 0;
    let mut ib = 0;
    let mut kk = k;
    loop {
        // If one array is exhausted, return from the other.
        if ia == a.len() {
            return b[ib + kk - 1];
        }
        if ib == b.len() {
            return a[ia + kk - 1];
        }
        // If k == 1, return the min of the current elements.
        if kk == 1 {
            return if a[ia] < b[ib] { a[ia] } else { b[ib] };
        }
        // Divide k into two parts.
        let pa = std::cmp::min(a.len() - ia, kk / 2);
        let pb = std::cmp::min(b.len() - ib, kk / 2);
        match a[ia + pa - 1].cmp(&b[ib + pb - 1]) {
            Ordering::Less => {
                ia += pa;
                kk -= pa;
            }
            _ => {
                ib += pb;
                kk -= pb;
            }
        }
    }
}

fn main() {
    let a = [1, 3, 5, 7, 9];
    let b = [2, 4, 6, 8, 10];
    for k in 1..=10 {
        let val = kth_smallest(&a, &b, k);
        println!("k={}: {}", k, val);
    }
}
