/// Finds the median of two sorted slices `nums1` and `nums2`.
///
/// This function implements a binary search approach to find the median in O(log(min(m, n))) time complexity,
/// where m and n are the lengths of the slices.
///
/// The core idea is to partition the two slices such that:
/// 1. The total number of elements in the left partitions equals the total number of elements in the right partitions.
/// 2. All elements in the left partitions are less than or equal to all elements in the right partitions.
///
/// Let `m` be the length of `nums1` and `n` be the length of `nums2`.
/// We perform binary search on the smaller slice (say `nums1`) to find the partition point `partition1`.
/// The corresponding partition point for `nums2` is then determined by `partition2 = (m + n + 1) / 2 - partition1`.
///
/// We need to satisfy the condition: `maxLeft1 <= minRight2` and `maxLeft2 <= minRight1`.
/// If these conditions are met, we've found the correct partitions.
/// The median is then calculated based on whether the total number of elements (m + n) is odd or even.
/// If odd, the median is `max(maxLeft1, maxLeft2)`.
/// If even, the median is `(max(maxLeft1, maxLeft2) + min(minRight1, minRight2)) / 2.0`.
///
/// Edge cases (e.g., partitions at the beginning or end of slices) are handled using `i32::MIN` and `i32::MAX`.
///
/// # Arguments
///
/// * `nums1` - The first sorted slice of `i32`.
/// * `nums2` - The second sorted slice of `i32`.
///
/// # Returns
///
/// The median of the two sorted slices as a `f64`.
pub fn find_median_sorted_arrays(nums1: &[i32], nums2: &[i32]) -> f64 {
    let (arr1, arr2) = if nums1.len() <= nums2.len() { (nums1, nums2) } else { (nums2, nums1) };

    let m = arr1.len();
    let n = arr2.len();
    let total_left_size = (m + n + 1) / 2;

    let mut low = 0;
    let mut high = m;

    while low <= high {
        let partition1 = low + (high - low) / 2;
        let partition2 = total_left_size - partition1;

        let max_left1 = if partition1 == 0 { i32::MIN } else { arr1[partition1 - 1] };
        let min_right1 = if partition1 == m { i32::MAX } else { arr1[partition1] };

        let max_left2 = if partition2 == 0 { i32::MIN } else { arr2[partition2 - 1] };
        let min_right2 = if partition2 == n { i32::MAX } else { arr2[partition2] };

        if max_left1 <= min_right2 && max_left2 <= min_right1 {
            // Correct partitions found
            if (m + n) % 2 == 0 { // Even number of elements
                (std::cmp::max(max_left1, max_left2) as f64 + std::cmp::min(min_right1, min_right2) as f64) / 2.0
            } else { // Odd number of elements
                std::cmp::max(max_left1, max_left2) as f64
            }
        } else if max_left1 > min_right2 {
            // partition1 is too large, move left in arr1
            high = partition1 - 1;
        } else { // max_left2 > min_right1
            // partition1 is too small, move right in arr1
            low = partition1 + 1;
        }
    }

    // This part should ideally not be reached if the input arrays are sorted.
    // Returning NaN or panicking might be more appropriate in a production scenario.
    f64::NAN
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median_odd_total() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(&nums1, &nums2), 2.0);
    }

    #[test]
    fn test_median_even_total() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(&nums1, &nums2), 2.5);
    }

    #[test]
    fn test_median_empty_first() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(find_median_sorted_arrays(&nums1, &nums2), 1.0);
    }

    #[test]
    fn test_median_empty_second() {
        let nums1 = vec![2];
        let nums2 = vec![];
        assert_eq!(find_median_sorted_arrays(&nums1, &nums2), 2.0);
    }

    #[test]
    fn test_median_complex_case() {
        let nums1 = vec![1, 3, 8, 9, 15];
        let nums2 = vec![7, 11, 18, 19, 21, 25];
        // Merged: [1, 3, 7, 8, 9, 11, 15, 18, 19, 21, 25] (11 elements)
        // Median is the 6th element: 11
        assert_eq!(find_median_sorted_arrays(&nums1, &nums2), 11.0);
    }

    #[test]
    fn test_median_complex_even_case() {
        let nums1 = vec![23, 26, 31, 35];
        let nums2 = vec![3, 5, 7, 9, 11, 16];
        // Merged: [3, 5, 7, 9, 11, 16, 23, 26, 31, 35] (10 elements)
        // Median is (11 + 16) / 2 = 13.5
        assert_eq!(find_median_sorted_arrays(&nums1, &nums2), 13.5);
    }
}