/// Finds the smallest missing positive integer in an unsorted array.
///
/// The array can contain duplicates, negative numbers, and zeros.
/// This algorithm uses the array itself as a hash map to mark the presence of positive integers.
/// It aims for O(n) time complexity and O(1) space complexity (modifying the input array in-place).
///
/// The core idea is to place each positive number `x` in the array at index `x-1`.
/// After rearranging, we iterate through the array to find the first index `i` where `nums[i] != i+1`.
/// That `i+1` is the smallest missing positive integer.
///
/// # Arguments
///
/// * `nums` - A mutable slice of `i32` integers.
///
/// # Returns
///
/// The smallest missing positive integer.
///
/// # Examples
///
/// ```
/// let mut nums = vec![1, 2, 0];
/// assert_eq!(find_first_missing_positive(&mut nums), 3);
///
/// let mut nums = vec![3, 4, -1, 1];
/// assert_eq!(find_first_missing_positive(&mut nums), 2);
///
/// let mut nums = vec![7, 8, 9, 11, 12];
/// assert_eq!(find_first_missing_positive(&mut nums), 1);
/// ```
pub fn find_first_missing_positive(nums: &mut [i32]) -> i32 {
    let n = nums.len();

    // Phase 1: Rearrange the array.
    // Place each positive number `x` at index `x-1` if possible.
    for i in 0..n {
        // Continue swapping as long as:
        // 1. nums[i] is positive.
        // 2. nums[i] is within the bounds of the array indices (1 to n).
        // 3. nums[i] is not already in its correct position (nums[nums[i] as usize - 1] != nums[i]).
        //    We use `as usize` for indexing and `nums[i] - 1` because the target index is 0-based.
        while nums[i] > 0 && nums[i] <= n as i32 && nums[(nums[i] - 1) as usize] != nums[i] {
            let swap_idx = (nums[i] - 1) as usize;
            nums.swap(i, swap_idx);
        }
    }

    // Phase 2: Find the first index `i` where `nums[i] != i+1`.
    for i in 0..n {
        if nums[i] != (i + 1) as i32 {
            return (i + 1) as i32;
        }
    }

    // If all numbers from 1 to n are present in their correct positions,
    // then the smallest missing positive is n+1.
    (n + 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_missing_positive_basic() {
        let mut nums = vec![1, 2, 0];
        assert_eq!(find_first_missing_positive(&mut nums), 3);
    }

    #[test]
    fn test_missing_positive_with_negatives() {
        let mut nums = vec![3, 4, -1, 1];
        assert_eq!(find_first_missing_positive(&mut nums), 2);
    }

    #[test]
    fn test_missing_positive_no_positives() {
        let mut nums = vec![-1, -2, -3];
        assert_eq!(find_first_missing_positive(&mut nums), 1);
    }

    #[test]
    fn test_missing_positive_all_present() {
        let mut nums = vec![1, 2, 3, 4];
        assert_eq!(find_first_missing_positive(&mut nums), 5);
    }

    #[test]
    fn test_missing_positive_duplicates() {
        let mut nums = vec![1, 1];
        assert_eq!(find_first_missing_positive(&mut nums), 2);
    }

    #[test]
    fn test_missing_positive_large_numbers() {
        let mut nums = vec![7, 8, 9, 11, 12];
        assert_eq!(find_first_missing_positive(&mut nums), 1);
    }

    #[test]
    fn test_missing_positive_empty() {
        let mut nums: Vec<i32> = vec![];
        assert_eq!(find_first_missing_positive(&mut nums), 1);
    }
}