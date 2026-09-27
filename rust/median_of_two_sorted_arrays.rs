use std::cmp::Ordering;

// Find median of two sorted slices in O(log(min(m, n))) time.
fn find_median_sorted_arrays(a: &[i32], b: &[i32]) -> f64 {
    let (mut x, mut y) = (a, b);
    if x.len() > y.len() {
        return find_median_sorted_arrays(y, x); // ensure x is smaller
    }
    let (mut low, mut high, mut m1, mut m2) = (0, x.len(), 0, 0);
    let total = x.len() + y.len();
    while low <= high {
        let partition_x = (low + high) / 2;
        let partition_y = (total + 1) / 2 - partition_x;

        let max_left_x = if partition_x == 0 { i32::MIN } else { x[partition_x - 1] };
        let min_right_x = if partition_x == x.len() { i32::MAX } else { x[partition_x] };
        let max_left_y = if partition_y == 0 { i32::MIN } else { y[partition_y - 1] };
        let min_right_y = if partition_y == y.len() { i32::MAX } else { y[partition_y] };

        match (max_left_x.cmp(&min_right_y), max_left_y.cmp(&min_right_x)) {
            (Ordering::Greater, _) => high = partition_x - 1,
            (_, Ordering::Greater) => low = partition_x + 1,
            _ => {
                m1 = if max_left_x > max_left_y { max_left_x } else { max_left_y };
                m2 = if min_right_x < min_right_y { min_right_x } else { min_right_y };
                break;
            }
        }
    }

    if total % 2 == 0 { (m1 as f64 + m2 as f64) / 2.0 } else { m1 as f64 }
}

fn main() {
    let a = [1, 3, 8, 9, 15];
    let b = [7, 11, 18, 19, 21, 25];
    println!("Median: {}", find_median_sorted_arrays(&a, &b));
}
