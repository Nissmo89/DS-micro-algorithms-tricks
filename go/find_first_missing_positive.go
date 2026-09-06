package main

// FindFirstMissingPositive finds the smallest missing positive integer in an unsorted array.
// The array can contain duplicates, negative numbers, and zeros.
// The algorithm uses the array itself as a hash map to mark the presence of positive integers.
// It aims for O(n) time complexity and O(1) space complexity (modifying the input array in-place).
//
// The core idea is to place each positive number `x` in the array at index `x-1`.
// After rearranging, we iterate through the array to find the first index `i` where `nums[i] != i+1`.
// That `i+1` is the smallest missing positive integer.
//
// Example:
// nums = [1, 2, 0]
// After rearrangement: [1, 2, 0] (0 is ignored, 1 is at index 0, 2 is at index 1)
// First index `i` where `nums[i] != i+1`: index 2 (nums[2]=0 != 2+1=3). Missing positive is 3.
//
// Example:
// nums = [3, 4, -1, 1]
// 1. Rearrange: Try to put 1 at index 0, 3 at index 2, 4 at index 3.
//    - nums[0]=3. Swap nums[0] with nums[2] -> [-1, 4, 3, 1]
//    - nums[0]=-1. Ignore.
//    - nums[1]=4. Swap nums[1] with nums[3] -> [-1, 1, 3, 4]
//    - nums[1]=1. Swap nums[1] with nums[0] -> [1, -1, 3, 4]
//    - nums[1]=-1. Ignore.
//    - nums[2]=3. Correct position.
//    - nums[3]=4. Correct position.
// Final rearranged (conceptually): [1, -1, 3, 4]
// Check: nums[0]=1 (correct), nums[1]=-1 (incorrect, expected 2). Smallest missing positive is 2.
func FindFirstMissingPositive(nums []int) int {
	n := len(nums)

	// Phase 1: Rearrange the array.
	// Place each positive number `x` at index `x-1` if possible.
	for i := 0; i < n; i++ {
		// Continue swapping as long as:
		// 1. nums[i] is positive.
		// 2. nums[i] is within the bounds of the array indices (1 to n).
		// 3. nums[i] is not already in its correct position (nums[nums[i]-1] != nums[i]).
		for nums[i] > 0 && nums[i] <= n && nums[nums[i]-1] != nums[i] {
			swapIndex := nums[i] - 1
			nums[i], nums[swapIndex] = nums[swapIndex], nums[i]
		}
	}

	// Phase 2: Find the first index `i` where `nums[i] != i+1`.
	for i := 0; i < n; i++ {
		if nums[i] != i+1 {
			return i + 1
		}
	}

	// If all numbers from 1 to n are present in their correct positions,
	// then the smallest missing positive is n+1.
	return n + 1
}
