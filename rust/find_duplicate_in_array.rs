/// Finds the duplicate number in an array `nums` which contains n+1 integers
/// where each integer is in the range [1, n] inclusive. It is guaranteed that
/// exactly one integer is repeated.
///
/// This algorithm uses the array indices and values to create a linked list structure.
/// The value at index `i` (`nums[i]`) represents the next node's index.
/// Since there's a duplicate, this structure forms a cycle.
/// Floyd's Tortoise and Hare algorithm is used to detect and find the start of this cycle,
/// which corresponds to the duplicate number.
///
/// IMPORTANT: This implementation modifies the input array by using values as indices.
/// The values in the array are in the range [1, n], and the array size is n+1.
/// This allows `nums[i]` to be used directly as an index.
///
/// # Arguments
///
/// * `nums` - A mutable slice of `i32` integers where one number is duplicated and others are unique in range [1, n].
///
/// # Returns
///
/// The duplicate number.
///
/// # Time Complexity
/// O(n)
///
/// # Space Complexity
/// O(1) (modifies input array)
pub fn find_duplicate_in_place(nums: &mut [i32]) -> i32 {
    // Phase 1: Find the intersection point of the two pointers.
    // Tortoise moves one step, hare moves two steps.
    let mut tortoise = nums[0];
    let mut hare = nums[0];

    loop {
        tortoise = nums[tortoise as usize];
        hare = nums[nums[hare as usize] as usize];
        if tortoise == hare {
            break;
        }
    }

    // Phase 2: Find the entrance to the cycle.
    // Reset tortoise to the start (index 0).
    // Move both tortoise and hare one step at a time until they meet.
    // The meeting point is the duplicate number.
    tortoise = nums[0];
    while tortoise != hare {
        tortoise = nums[tortoise as usize];
        hare = nums[hare as usize];
    }

    hare // or tortoise, they meet at the duplicate number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_duplicate_basic() {
        let mut nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate_in_place(&mut nums), 2);
    }

    #[test]
    fn test_find_duplicate_another() {
        let mut nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate_in_place(&mut nums), 3);
    }

    #[test]
    fn test_find_duplicate_small() {
        let mut nums = vec![1, 1];
        assert_eq!(find_duplicate_in_place(&mut nums), 1);
    }

    #[test]
    fn test_find_duplicate_larger() {
        let mut nums = vec![2, 5, 9, 6, 9, 3, 8, 9, 7, 1];
        assert_eq!(find_duplicate_in_place(&mut nums), 9);
    }
}