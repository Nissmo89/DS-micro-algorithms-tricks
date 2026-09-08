package main

// reverse reverses the elements of a slice within the given range [start, end].
func reverse(nums []int, start, end int) {
	for start < end {
		nums[start], nums[end] = nums[end], nums[start]
		start++
		end--
	}
}

// RotateRightOptimized rotates the elements of the slice `nums` to the right by `k` steps.
// This implementation uses the reversal algorithm, which is efficient (O(n) time, O(1) space).
// The algorithm works in three steps:
// 1. Reverse the entire array.
// 2. Reverse the first k elements.
// 3. Reverse the remaining n-k elements.
// This optimized version ensures k is handled correctly for negative values and multiples of n.
func RotateRightOptimized(nums []int, k int) {
	n := len(nums)
	if n == 0 {
		return
	}

	// Normalize k: handle cases where k is larger than n or negative.
	k = k % n
	if k < 0 {
		k += n // Ensure k is positive after modulo
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
