package main

// reverse reverses the elements of a slice within the given range [start, end].
func reverse(nums []int, start, end int) {
	for start < end {
		nums[start], nums[end] = nums[end], nums[start]
		start++
		end--
	}
}

// RotateRight rotates the elements of the slice `nums` to the right by `k` steps.
// This implementation uses the reversal algorithm, which is efficient (O(n) time, O(1) space).
// The algorithm works in three steps:
// 1. Reverse the entire array.
// 2. Reverse the first k elements.
// 3. Reverse the remaining n-k elements.
//
// Example:
// nums = [1, 2, 3, 4, 5, 6, 7], k = 3
// 1. Reverse all: [7, 6, 5, 4, 3, 2, 1]
// 2. Reverse first k=3: [5, 6, 7, 4, 3, 2, 1]
// 3. Reverse remaining n-k=4: [5, 6, 7, 1, 2, 3, 4]
func RotateRight(nums []int, k int) {
	n := len(nums)
	if n == 0 {
		return
	}

	// Handle cases where k is larger than n or negative.
	k = k % n
	if k < 0 {
		k += n
	}

	if k == 0 {
		return // No rotation needed
	}

	// Step 1: Reverse the entire array
	reverse(nums, 0, n-1)

	// Step 2: Reverse the first k elements
	reverse(nums, 0, k-1)

	// Step 3: Reverse the remaining n-k elements
	reverse(nums, k, n-1)
}
