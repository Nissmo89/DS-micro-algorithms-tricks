pub fn first_missing_positive(nums: &mut Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n {
        while nums[i] > 0
            && (nums[i] as usize) <= n
            && nums[nums[i] as usize - 1] != nums[i]
        {
            let target = nums[i] as usize - 1;
            nums.swap(i, target);
        }
    }
    for i in 0..n {
        if nums[i] != (i as i32 + 1) {
            return i as i32 + 1;
        }
    }
    (n as i32) + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_first_missing_positive() {
        let mut v = vec![3, 4, -1, 1];
        assert_eq!(first_missing_positive(&mut v), 2);
        let mut v2 = vec![1, 2, 0];
        assert_eq!(first_missing_positive(&mut v2), 3);
    }
}