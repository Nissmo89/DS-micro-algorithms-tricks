pub fn max_subarray_sum_with_one_deletion(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }
    let mut inc = vec![0; n]; // max sum ending at i without deletion
    let mut dec = vec![0; n]; // max sum ending at i with one deletion
    inc[0] = nums[0];
    dec[0] = std::i32::MIN; // impossible to delete at first element
    let mut best = nums[0];
    for i in 1..n {
        inc[i] = std::cmp::max(nums[i], inc[i - 1] + nums[i]);
        // either delete current element (take inc[i-1]) or continue a deletion
        dec[i] = std::cmp::max(inc[i - 1], dec[i - 1] + nums[i]);
        best = std::cmp::max(best, std::cmp::max(inc[i], dec[i]));
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_examples() {
        assert_eq!(max_subarray_sum_with_one_deletion(&[1, -2, 0, 3]), 4); // delete -2
        assert_eq!(max_subarray_sum_with_one_deletion(&[1, -2, -2, 3]), 3); // delete one -2
        assert_eq!(max_subarray_sum_with_one_deletion(&[-1, -2, -3]), -1); // all negative
    }
}
